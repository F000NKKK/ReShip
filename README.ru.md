# ReShip

**Языки:** [English](README.md) | Русский

ReShip — распределённая платформа доставки релизов и обновлений приложений. CI один раз публикует immutable release resources; клиент через RSP определяет desired release, выбирает manifest своего target, скачивает только недостающий verified content, собирает чистый snapshot установки и активирует его через rollback-safe agent boundary.

> **Статус:** bootstrap / архитектура v0.1. RSP foundation и границы проекта уже заданы; publication, persistence, client agent, SDK, regional delivery, trust metadata и activation реализуются отдельными задачами `RS-*`.

## Основные свойства

- единый pipeline релиза от CI до клиента;
- immutable release descriptors, manifests и content-addressed artifacts;
- чистые snapshots: удалённый из релиза файл не переживает обновление;
- file-level differential network delivery без destructive in-place update;
- CDN необязателен и не меняет RSP semantics;
- региональное масштабирование с fallback на origin;
- semantic crates RSP не зависят от HTTP;
- одна Rust-реализация updater/agent за тонкими language SDK;
- server и protocol application-neutral.

## Структура

```text
protocol/                 RSP — отдельный вложенный Rust workspace
  rsp-core                protocol primitives, digests, ChannelRevision, TargetId
  rsp-manifest            immutable per-target snapshot установки
  rsp-release             immutable logical release descriptors
  rsp-discovery           mutable desired state application/channel
  rsp-json                canonical JSON bytes и canonical SHA-256 identity
  rsp-http                HTTP/CDN binding и delivery topology

crates/
  reship-server           control plane / origin API
  reship-storage          storage boundary
  reship-cli              CLI для CI и операторов

sdk/                      thin language SDK contracts
```

Направление зависимостей строгое: ReShip и client/SDK layer могут зависеть от RSP; RSP никогда не зависит от `reship-*`.

## Release model

```text
ChannelState (mutable, ChannelRevision)
    -> ReleaseDescriptor (immutable)
        -> ReleaseManifest per TargetId (immutable)
            -> artifacts by ContentDigest (immutable)
```

Строка версии приложения — display metadata и не задаёт порядок релизов. Promotion и rollback меняют только channel pointer и увеличивают `ChannelRevision`; immutable release resources при этом не пересобираются.

Canonical JSON resources хэшируются только из deterministic canonical bytes. SHA-256 identity имеет форму `sha256:<64 lowercase hex>`.

## Delivery model

Channel state маленький и revalidate-ится через ETag. Release descriptors, manifests и artifacts immutable и digest-addressed, поэтому подходят для длинного CDN caching. Delivery topology отделена от channel state, потому что регион клиента или availability edge могут меняться независимо от release.

```text
CI ── reship publish ──► Control plane
                           │
                           ├─ channel state (mutable, ETag)
                           ├─ delivery topology (mutable)
                           └─ descriptor/manifests/artifacts (immutable CAS)
                                             │
                                  ┌──────────┴──────────┐
                                  │                     │
                                 CDN                  Origin
                                  │                     │
                                  └──────────┬──────────┘
                                             ▼
                                      Rust client agent
                                             │
                                      verified local CAS
                                             │
                                      staged release tree
                                             │
                                       activation pointer
                                             │
                                        application
```

Подробности — в [ARCHITECTURE.ru.md](ARCHITECTURE.ru.md).
