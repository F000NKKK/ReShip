# ReShip

**Languages:** English | [Русский](README.ru.md)

ReShip is a distributed application release and update platform. CI publishes immutable release resources once; clients discover the desired release through RSP, resolve the target manifest, download only missing verified content, materialize a clean installation snapshot, and activate it through a rollback-safe agent boundary.

> **Status:** bootstrap / v0.1 architecture. The RSP foundation and project boundaries exist; publication, persistence, client agent, SDKs, regional delivery, trust metadata, and activation are implemented in bounded follow-up `RS-*` tasks.

## Why

ReShip replaces update pipelines that couple CI, application-specific storage APIs, schedulers, message brokers, push hubs, custom TCP transports, and in-place file copying. Update availability is modeled as durable channel state rather than a one-shot event.

Core properties:

- one release pipeline from CI to every client;
- immutable release descriptors, manifests, and content-addressed artifacts;
- clean snapshots, so removed files cannot survive an update;
- file-level differential network delivery without destructive in-place updates;
- optional CDN with the same RSP HTTP binding and origin fallback;
- regional delivery without changing application release semantics;
- transport-neutral RSP semantic crates;
- one Rust updater/agent implementation behind thin language SDKs;
- application-neutral server and protocol.

## Repository layout

```text
protocol/                 RSP — independent nested Rust workspace
  rsp-core                protocol primitives, digests, channel revision, target IDs
  rsp-manifest            immutable per-target installation snapshots
  rsp-release             immutable logical release descriptors
  rsp-discovery           mutable application/channel desired state
  rsp-json                canonical JSON bytes and canonical SHA-256 identities
  rsp-http                HTTP/CDN binding and delivery topology

crates/                   ReShip implementation workspace
  reship-server           control plane / origin API
  reship-storage          storage boundary
  reship-cli              CI/operator CLI

sdk/                      thin language SDK contracts and future implementations
```

The dependency direction is strict:

```text
RSP  <── client/SDK layer
 ↑
 └──── ReShip server / CLI / storage
```

RSP must never depend on `reship-*`.

## Release model

The authoritative resource graph is:

```text
ChannelState (mutable, ChannelRevision)
    -> ReleaseDescriptor (immutable)
        -> ReleaseManifest per TargetId (immutable)
            -> artifacts by ContentDigest (immutable)
```

Human application versions are display metadata and do not order releases. Promotion and rollback only mutate the channel pointer and increment `ChannelRevision`; immutable release resources are never rebuilt during promotion.

Canonical JSON resources are hashed from deterministic canonical bytes. SHA-256 identities use `sha256:<64 lowercase hex>`.

## Delivery model

Channel state is small and conditionally revalidated with ETag. Release descriptors, manifests, and artifacts are immutable and digest-addressed, so they can use long CDN cache lifetimes. Delivery topology is separate from channel state because region/edge availability may change independently of a release.

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
