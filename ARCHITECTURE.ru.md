# Архитектура ReShip

**Языки:** [English](ARCHITECTURE.md) | Русский

## Цель v0.1

Создать небольшую и надёжную платформу доставки релизов, которая заменяет application-specific update chains, но не превращается в новый набор из десятка микросервисов.

У ReShip три плоскости:

1. **RSP protocol plane** — независимо версионируемая семантика и bindings.
2. **Control plane** — публикация, приложения, каналы, metadata, policy и выбор delivery endpoints.
3. **Data plane** — immutable release descriptors, manifests и artifacts через origin, CDN или региональные mirrors.

Updater/agent/SDK являются клиентами этих плоскостей и не входят в control plane.

## Жёсткие инварианты

1. Опубликованные release resources immutable.
2. Release descriptor идентифицирует один логический release приложения и связывает target IDs с immutable manifests.
3. Manifest описывает полный snapshot установки для одного target, а не список файлов «скопировать поверх».
4. Artifacts content-addressed и проверяются до activation.
5. Channel state mutable, а release descriptors/manifests/artifacts immutable.
6. Наличие обновления — долговечное состояние, а не одноразовый event.
7. RSP application-neutral и infrastructure-neutral.
8. RSP никогда не зависит от `reship-*`.
9. HTTP — binding RSP, а не сам semantic protocol.
10. CDN необязателен; origin-only deployment обязан работать тем же updater.
11. Региональное масштабирование меняет delivery endpoints, а не release/manifest semantics.
12. CI публикует release и не участвует в runtime update path.
13. Activation строит чистый target snapshot и сохраняет rollback boundary.

## Декомпозиция RSP

`protocol/` — отдельный вложенный Cargo workspace со своей metadata и cadence версий.

```text
rsp-core
   ↑
   ├──── rsp-manifest
   ├──── rsp-release
   └──── rsp-discovery

rsp-json  -> canonical JSON bytes + canonical SHA-256 identities
rsp-http  -> только HTTP binding
```

`rsp-core` — версия протокола, IDs, canonical content digests, `ChannelRevision`, content lengths и target identifiers.

`rsp-manifest` — immutable filesystem snapshot для одного `TargetId`, без URL/storage деталей.

`rsp-release` — immutable `ReleaseDescriptor`, который связывает identity/version логического release с map `TargetId -> manifest digest`.

`rsp-discovery` — mutable `ChannelState`. Channel указывает на digest release descriptor и содержит monotonic `ChannelRevision`.

`rsp-json` — deterministic canonical JSON serialization и authoritative SHA-256 digest canonical JSON resources.

`rsp-http` — HTTP media types, conditional polling, immutable digest paths, cache-control guidance и delivery topology. Другой binding не должен требовать изменений semantic crates.

## Release identity и порядок channel state

Строка версии приложения — только отображаемая metadata и не задаёт порядок релизов.

Immutable release имеет identity, а не глобальный sequence:

```text
ChannelState (mutable)
    -> ReleaseDescriptor (immutable)
        -> ReleaseManifest per TargetId (immutable)
            -> artifacts by ContentDigest (immutable)
```

Упорядочиваются только изменения channel pointer. `ChannelRevision` монотонно растёт внутри одной application/channel identity и увеличивается как при promotion, так и при rollback. Поэтому rollback может вернуть channel на старый immutable release descriptor, но всё равно создаёт более новый revision.

Promotion между channels не пересобирает и не изменяет descriptor, manifests или artifacts.

## Canonical wire representation

Canonical JSON — вход для hashing/signing JSON-ресурсов RSP. Используется deterministic UTF-8 JSON со стабильной сортировкой ключей по UTF-16. Floating-point значения и integers за пределами точного binary64 range запрещены. Большие 64-bit значения протокола, например content size и channel revision, кодируются canonical decimal strings.

SHA-256 identity имеет вид:

```text
sha256:<64 lowercase hexadecimal digits>
```

Publisher и client должны хэшировать canonical RSP bytes, а не произвольный вывод конкретного serializer.

## Snapshot и differential delivery

Manifest описывает все managed regular files целевой установки. Для network planning можно сравнивать installed и target manifests:

```text
тот же path + тот же digest -> content уже доступен
новый digest/new path       -> content необходимо получить
path отсутствует в target   -> не materialize в новый snapshot
```

Authoritative activation не выполняет destructive `DELETE` непосредственно в активной установке. Updater materialize-ит новый versioned release tree из verified content, после чего меняет active pointer через platform activation boundary.

Binary delta не является обязательной частью RSP v1. Позже его можно добавить как optional artifact representation, не меняя authoritative target manifest.

## Portable manifest paths

RSP v1 использует консервативную cross-platform path grammar. Пути относительные, используют `/` и запрещают traversal, empty segments, backslashes, control characters, Windows alternate-stream syntax, недопустимые Windows characters, trailing dot/space и reserved device names.

Platform materializer дополнительно обязан обнаруживать коллизии путей из-за case folding или Unicode normalization конкретной файловой системы. Symlink/reparse-point semantics не входят в v1 regular-file manifest model.

## Publication semantics

Publication и channel promotion — разные операции:

1. загрузить immutable artifacts и проверить digests;
2. построить и validate canonical target manifests;
3. сохранить manifests по canonical digest;
4. построить один canonical release descriptor, ссылающийся на manifests;
5. сохранить descriptor по canonical digest;
6. зарегистрировать release metadata;
7. отдельной compare-and-swap операцией продвинуть channel pointer.

Неудачная публикация может оставить unreachable immutable objects; позже их удалит GC. Это безопаснее, чем имитировать distributed transaction между metadata store и object storage.

Promotion использует expected channel revision. Concurrent mutation должна завершаться conflict и не может тихо затереть более новый channel state.

## Control plane

Первая версия — один deployable Rust service. Внутренние модули не становятся отдельными микросервисами без измеримой операционной причины.

Он отвечает за:

- applications/channels;
- authenticated publication;
- validation paths, uniqueness и digests;
- immutable release descriptors/manifests/artifacts;
- compare-and-swap channel pointers/revisions;
- RSP discovery;
- delivery endpoint selection;
- health/metrics/audit.

Metadata database и artifact storage скрыты за implementation boundaries. Local development может использовать filesystem + SQLite, production — S3-compatible object storage + PostgreSQL без изменений RSP.

## CDN и data plane

Immutable resources используют digest-addressed paths:

```text
/rsp/v1/releases/<algorithm>/<digest>
/rsp/v1/manifests/<algorithm>/<digest>
/rsp/v1/artifacts/<algorithm>/<digest>
```

Для них допустимы длинные public cache lifetimes, потому что representation по digest URL никогда не меняется. RSP HTTP рекомендует `public, max-age=31536000, immutable, no-transform`.

Channel state:

```text
/rsp/v1/apps/<app>/channels/<channel>/state
```

Он mutable и revalidate-ится через ETag/`If-None-Match` с `Cache-Control: no-cache` semantics.

Delivery topology живёт отдельно:

```text
/rsp/v1/apps/<app>/delivery
```

Регион клиента или доступность edge могут измениться без изменения channel state, поэтому endpoints не входят в `ChannelState`.

При resumable artifact download клиент корректно обрабатывает `206 Content-Range`; ответ `200` означает restart partial download. Полный digest всегда проверяется до помещения artifact в local CAS.

## Multi-region

Для v0.1 достаточно одного authoritative control-plane writer и регионально/глобально replicated immutable data plane. Active-active metadata writes не нужны до появления конкретной необходимости.

Без внешнего CDN тот же server binary позже может поддерживать read-only edge role, который cache/serve release descriptors, manifests и artifacts. Edge не публикует releases, не двигает channels и не владеет signing keys.

## Client, agent и SDK

Secure updater не должен независимо переписываться в каждом language SDK.

Целевая граница:

```text
language SDK
    -> local versioned client API
        -> reship-agent / reship-client-core (Rust)
            -> RSP
```

Rust client core/agent отвечает за discovery, canonical/trust verification, local CAS, resumable downloads, snapshot planning/materialization, activation state, rollback, recovery и GC. Language SDK — тонкая typed integration с local client API и lifecycle приложения.

Agent может быть command/mode-based bootstrap executable и не обязан постоянно работать как daemon.

Installed application files не считаются trusted CAS. Сначала verified blobs попадают в local CAS, затем из него строится clean release tree. Writable release files нельзя hard-link-ить напрямую с trusted CAS objects, иначе приложение сможет изменить доверенный контент.

## Activation state

Agent находится вне managed application release tree. Local state хранит как минимум current, previous known-good и pending release identities, а также accepted channel/trust state.

```text
stage -> activate pointer -> launch -> application confirms -> keep
                               │
                               └─ timeout/crash -> recover previous known-good
```

Так мы не притворяемся, что замена непустой работающей Windows installation directory всегда является атомарной операцией.

## Security baseline

- TLS для всех network paths.
- Publisher credentials отделены от client download policy.
- SHA-256 content verification обязательна.
- Canonical JSON — единственный hashing/signing input для canonical JSON resources.
- Path validation исключает опасные cross-platform materialization semantics.
- Release descriptors/manifests/artifacts immutable после commit.
- Digest verification даёт integrity, но не publisher authenticity и не freshness.
- Production trust/signing — отдельный security layer; он должен быть rollback/freeze-resistant, а не ad-hoc signature field.

## Первые этапы

1. RSP foundation: release descriptor, channel revision, canonical JSON/digest, paths, HTTP resource semantics и fixtures.
2. Local filesystem CAS + metadata persistence.
3. `reship publish` + compare-and-swap channel promotion.
4. Read-only RSP HTTP API + ETag/range.
5. Rust client core/agent + local client API.
6. Thin .NET SDK integration.
7. Windows activation/bootstrap recovery flow.
8. Production trust/signing metadata.
9. S3-compatible origin + CDN/edge deployment.
10. Metrics/telemetry и policy layers.

Каждый этап оформляется отдельными задачами `RS-*` в YouTrack.
