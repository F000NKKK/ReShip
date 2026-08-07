# ReShip

**Languages:** English | [Русский](README.ru.md)

ReShip is a distributed application release and update platform. CI publishes an immutable release once; clients discover the desired release through RSP, download only missing content, construct a clean installation snapshot, and activate it atomically.

> **Status:** bootstrap / v0.1 architecture. The protocol model and project boundaries exist; publication, persistence, SDKs, regional routing, and activation are intentionally implemented in bounded follow-up `RS-*` tasks.

## Why

ReShip replaces update pipelines that couple CI, application-specific storage APIs, schedulers, message brokers, push hubs, custom TCP transports, and in-place file copying. Update availability is modeled as durable channel state rather than a one-shot event.

Core properties:

- one release pipeline from CI to every client;
- immutable release manifests and content-addressed artifacts;
- clean snapshots, so removed files cannot survive an update;
- file-level differential downloads from the first version (`keep / download / delete`);
- optional CDN with the same RSP HTTP binding and origin fallback;
- regional delivery without changing application SDK semantics;
- transport-neutral protocol semantics;
- application-neutral server: ETS is a consumer, not a special case.

## Repository layout

```text
protocol/                 RSP — independent nested Rust workspace
  rsp-core                protocol primitives and versioning
  rsp-manifest            immutable installation snapshots
  rsp-discovery           application/channel desired state
  rsp-http                HTTP/CDN binding

crates/                   ReShip implementation workspace
  reship-server           control plane / origin API
  reship-storage          storage boundary
  reship-cli              CI/operator CLI

sdk/                      language SDK contracts and future implementations
```

The dependency direction is strict:

```text
RSP  <── SDKs
 ↑
 └──── ReShip server / CLI / storage
```

RSP must never depend on `reship-*`.

## Delivery model

Mutable channel state is small and conditionally polled with ETag. Manifests and artifacts are immutable and content-addressed, so they can be cached indefinitely by a CDN.

```text
CI ── reship publish ──► Control plane
                           │
                           ├─ channel state (mutable, ETag)
                           └─ manifest/artifacts (immutable CAS)
                                      │
                            ┌─────────┴─────────┐
                            │                   │
                           CDN               Origin
                            │                   │
                            └─────────┬─────────┘
                                      ▼
                                   Client SDK
                                      │
                               staged snapshot
                                      │
                                  bootstrapper
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for the v0.1 design.

## Development

Rust MSRV is 1.93. Both workspaces are checked independently:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps

cargo fmt --manifest-path protocol/Cargo.toml --all -- --check
cargo clippy --manifest-path protocol/Cargo.toml --workspace --all-targets --all-features -- -D warnings
cargo test --manifest-path protocol/Cargo.toml --workspace --all-features
cargo doc --manifest-path protocol/Cargo.toml --workspace --all-features --no-deps
```

Repository work is tracked in the YouTrack project **ReShip** with IDs `RS-*`. Read `AGENTS.md` for Codex/agent workflow and `CLAUDE.md` for Claude Code.

## License

MPL-2.0.
