# Архитектура ReShip

**Языки:** [English](ARCHITECTURE.md) | Русский

## Цель v0.1

Создать небольшую и надёжную платформу доставки релизов, которая заменяет application-specific update chains, но не превращается в новый набор из десятка микросервисов.

У ReShip три плоскости:

1. **RSP protocol plane** — независимо версионируемая семантика и bindings.
2. **Control plane** — публикация, приложения, каналы, metadata и выбор delivery endpoints.
3. **Data plane** — immutable manifests/artifacts через origin, CDN или региональные mirrors.

## Жёсткие инварианты

1. Опубликованный release immutable.
2. Manifest описывает полный snapshot установки, а не список файлов «скопировать поверх».
3. Artifacts content-addressed и проверяются до activation.
4. Channel state mutable, release content immutable.
5. Наличие обновления — долговечное состояние, а не одноразовый event.
6. RSP не знает ни о ETS, ни о БД, CDN, S3, регионе или ReShip Server.
7. RSP никогда не зависит от `reship-*`.
8. HTTP — binding RSP, а не сам протокол.
9. CDN необязателен; origin-only deployment обязан работать тем же SDK.
10. Региональное масштабирование меняет delivery endpoint, а не semantics manifests.
11. CI только публикует release и не участвует в runtime update path.
12. Activation строит чистый target snapshot и имеет rollback boundary.

## RSP

`protocol/` — отдельный вложенный Cargo workspace со своей версией и metadata:

```text
rsp-core
   ↑
   ├──── rsp-manifest
   └──── rsp-discovery
            ↑
            └──── rsp-http
```

`rsp-core` — версия протокола, IDs, digest, release sequence и target selector.

`rsp-manifest` — полный immutable filesystem snapshot без URL/storage деталей.

`rsp-discovery` — desired release для application/channel/target и minimum-supported sequence.

`rsp-http` — HTTP media types, conditional polling, CAS paths и ordered delivery endpoints. В будущем другой binding не должен требовать изменений semantic crates.

## Версии

Строка версии приложения — только отображаемая metadata. Для порядка используется монотонный `ReleaseSequence` внутри application/channel/target stream. Поэтому RSP не навязывает SemVer и подходит для legacy versioning ETS.

## Обновление

Сравнение installed manifest и target manifest даёт:

```text
тот же path + тот же digest -> KEEP
новый digest/new path       -> DOWNLOAD
path отсутствует в target   -> DELETE / не переносить в staging
```

Правильный путь — собрать новый staging snapshot и атомарно заменить текущую установку. Не обновлять директорию по файлам поверх старой.

Binary delta не является обязательной частью RSP v1. Позже его можно добавить как дополнительное представление artifact, не меняя authoritative target manifest.

## Control plane

Первая версия — один deployable Rust service. Внутренние модули не становятся отдельными микросервисами без реальной операционной причины.

Он отвечает за applications/channels, authenticated publication, immutable release metadata, atomic channel pointer, RSP discovery, delivery endpoint selection, health/metrics/audit.

## CDN и регионы

Manifest/artifact лежат по digest URL и могут кэшироваться CDN практически бессрочно. Channel state маленький, mutable и проверяется через ETag.

Без CDN endpoints ведут на origin. С CDN первым endpoint становится edge, origin остаётся fallback. Клиент всё равно проверяет SHA-256.

Для v0.1 достаточно одного authoritative control plane и регионально/глобально реплицируемого immutable data plane. Active-active запись metadata не нужна до появления измеримой необходимости.

## SDK

SDK выполняет discovery, manifest verification, строит `keep/download/delete`, скачивает artifacts с fallback, создаёт staging и хранит installed state. Замена запущенных файлов делегируется platform bootstrapper.

Первый production consumer — .NET SDK для ETS Client, но RSP остаётся language-neutral.

## Первые этапы

1. RSP validation/canonical JSON/fixtures.
2. Filesystem CAS + metadata persistence.
3. `reship publish` и promote.
4. RSP HTTP API + ETag/range.
5. .NET SDK.
6. Windows bootstrapper с atomic activation/rollback.
7. ETS Test integration.
8. S3-compatible origin + CDN.
9. Metrics/telemetry/minimum-supported policy.

Каждый этап оформляется отдельными задачами `RS-*` в YouTrack.
