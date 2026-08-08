# ReShip architecture

**Languages:** English | [Русский](ARCHITECTURE.ru.md)

## v0.1 goal

Establish a small, reliable release-distribution platform that can replace application-specific update chains without recreating them as a fleet of microservices.

ReShip v0.1 has three architectural planes:

1. **RSP protocol plane** — independently versioned semantics and bindings.
2. **Control plane** — publishing, applications, channels, release metadata, policy, and delivery selection.
3. **Data plane** — immutable release descriptors, manifests, and artifacts delivered from origin, CDN, or regional mirrors.

The updater/agent/SDK stack is a client of those planes, not part of the control plane.

## Hard invariants

1. Published release resources are immutable.
2. A release descriptor identifies one logical application release and maps target IDs to immutable manifests.
3. A release manifest is a complete installation snapshot for one target, not a list of copied-over files.
4. Artifacts are content-addressed and verified before activation.
5. Channel state is mutable; release descriptors, manifests, and artifacts are immutable.
6. Update availability is durable state, never a one-shot notification event.
7. RSP is application-neutral and infrastructure-neutral.
8. RSP has no dependency on ReShip implementation crates.
9. HTTP is an RSP binding, not RSP itself.
10. CDN is optional. The same updater must work against origin only.
11. Regional scaling changes delivery endpoints, not release or manifest semantics.
12. CI publishes; it is not part of the runtime update path.
13. Activation constructs a clean target snapshot and preserves rollback boundaries.

## RSP decomposition

`protocol/` is a nested Cargo workspace with its own package metadata and version cadence.

```text
rsp-core
   ↑
   ├──── rsp-manifest
   ├──── rsp-release
   └──── rsp-discovery

rsp-json  -> canonical JSON bytes + canonical SHA-256 identities
rsp-http  -> HTTP binding only
```

`rsp-core` owns protocol version, IDs, canonical content digests, `ChannelRevision`, content lengths, and target identifiers.

`rsp-manifest` owns the immutable filesystem snapshot for one `TargetId`. It does not prescribe storage URLs.

`rsp-release` owns the immutable logical `ReleaseDescriptor`. It binds one application release identity/version to a map of target IDs and manifest digests.

`rsp-discovery` owns mutable `ChannelState`. A channel points to a release descriptor digest and carries a monotonic `ChannelRevision`.

`rsp-json` owns deterministic canonical JSON serialization and the authoritative SHA-256 digest of canonical JSON resources.

`rsp-http` maps resources to HTTP media types, conditional channel-state polling, immutable digest paths, cache-control guidance, and delivery topology. A future gRPC or filesystem binding must not require changes to semantic crates.

## Release identity and channel ordering

Human release version strings are display metadata and are not trusted for ordering.

An immutable release has identity, not a global sequence. The resource chain is:

```text
ChannelState (mutable)
    -> ReleaseDescriptor (immutable)
        -> ReleaseManifest per TargetId (immutable)
            -> artifacts by ContentDigest (immutable)
```

Only channel mutations are ordered. `ChannelRevision` is monotonic within one application/channel identity and advances for both promotion and rollback. A rollback therefore points to an older immutable release descriptor while still producing a newer channel revision.

Promotion between channels must never rebuild or mutate the descriptor, manifests, or artifacts.

## Canonical wire representation

Canonical JSON is the hashing/signing input for JSON RSP resources. RSP uses deterministic UTF-8 JSON with stable UTF-16 key ordering and rejects floating-point values and integers outside the exact binary64 range. Large 64-bit protocol values such as content sizes and channel revisions are encoded as canonical decimal strings.

SHA-256 resource identities use:

```text
sha256:<64 lowercase hexadecimal digits>
```

Publishers and clients must hash canonical RSP bytes, not an arbitrary serializer representation.

## Snapshot and differential update

A manifest describes every managed regular file in the target installation tree. Network planning may compare installed and target manifests:

```text
same path + same digest   -> content already available
same/new path + new hash -> content required
old path absent in target -> not materialized in new snapshot
```

The authoritative activation model does not directly execute destructive `DELETE` operations against the active installation. The updater materializes a new versioned release tree from verified content, then changes the active pointer through the platform activation boundary.

Binary patches are explicitly out of scope for the first protocol version. They can later become an optional artifact representation while the target manifest remains authoritative.

## Portable manifest paths

RSP v1 uses a conservative cross-platform path grammar. Manifest paths are relative, forward-slash separated, and reject traversal, empty segments, backslashes, control characters, Windows alternate-stream syntax, invalid Windows characters, trailing dots/spaces, and reserved device names.

Platform materializers must additionally detect path collisions caused by destination-specific case-folding or Unicode-normalization behavior. Symlink/reparse-point semantics are not part of the v1 regular-file manifest model.

## Publication semantics

Publication and channel promotion are separate operations:

1. upload immutable artifacts and verify their digests;
2. build and validate canonical target manifests;
3. persist manifests by canonical digest;
4. build one canonical release descriptor referencing those manifests;
5. persist the descriptor by canonical digest;
6. register release metadata;
7. promote a channel pointer in a separate compare-and-swap operation.

A failed publication may leave unreachable immutable objects; garbage collection handles those later. It is preferable to orphan immutable blobs than to emulate a distributed transaction across metadata and object storage.

Promotion uses the expected channel revision. A concurrent mutation is a conflict and must not silently overwrite the newer channel state.

## Control plane

The first server is one deployable Rust service, not a microservice fleet. Logical modules may later be split only with operational evidence.

Responsibilities:

- register applications and channels;
- accept authenticated release publication;
- validate paths, uniqueness and digests;
- commit immutable release descriptors/manifests/artifacts;
- compare-and-swap channel pointers and revisions;
- expose RSP discovery resources;
- choose ordered delivery endpoints based on configuration/region;
- expose health/metrics and audit events.

A relational metadata database and artifact storage are implementation details behind boundaries. Local development may use filesystem + SQLite; production can use S3-compatible object storage + PostgreSQL without changing RSP.

## Data plane and CDN

Immutable resources use digest-addressed paths:

```text
/rsp/v1/releases/<algorithm>/<digest>
/rsp/v1/manifests/<algorithm>/<digest>
/rsp/v1/artifacts/<algorithm>/<digest>
```

They may use long public cache lifetimes because content at a digest URL never changes. RSP HTTP recommends `public, max-age=31536000, immutable, no-transform` for immutable resources.

Channel state is different:

```text
/rsp/v1/apps/<app>/channels/<channel>/state
```

It is mutable, small, and uses ETag/`If-None-Match` with `Cache-Control: no-cache` semantics so retained representations are revalidated before reuse.

Delivery topology is independently mutable:

```text
/rsp/v1/apps/<app>/delivery
```

A client may move regions or an edge may become unavailable without any channel mutation, so delivery endpoints are not embedded in `ChannelState`.

Artifact downloads may use HTTP ranges. A resumed transfer must handle `206 Content-Range` correctly; if the server answers `200`, the client restarts the partial download. The complete artifact digest is always verified before CAS admission.

## Multi-region

v0.1 does not require active-active metadata writes. The recommended starting topology is one authoritative control-plane writer plus globally/regionally replicated immutable data.

```text
                    authoritative control plane
                             │
                   metadata + origin CAS
                             │
           ┌─────────────────┼─────────────────┐
           ▼                 ▼                 ▼
        CDN/US            CDN/EU            CDN/APAC
           │                 │                 │
        clients           clients           clients
```

Without an external CDN, the same server binary may later provide a read-only edge role that caches and serves release descriptors, manifests, and artifacts. Edge roles do not publish, promote channels, or own signing keys.

Publication remains serialized until a concrete need justifies distributed write coordination.

## Client, agent, and SDK contract

Secure update behavior should have one implementation boundary rather than being reimplemented independently in every language SDK.

The intended client stack is:

```text
language SDK
    -> local versioned client API
        -> reship-agent / reship-client-core (Rust)
            -> RSP
```

The Rust client core/agent owns discovery, canonical/trust verification, local CAS, resumable downloads, snapshot planning/materialization, activation state, rollback, recovery, and garbage collection. Language SDKs are thin typed integrations around the local client API and application lifecycle.

The agent may run as a command/mode-based bootstrap executable; it is not required to be a permanent daemon.

Installed application files are not trusted as CAS content. Verified blobs enter a local CAS, then a clean release tree is materialized from that CAS. Hard-linking writable release files directly to trusted CAS objects is forbidden because application writes could mutate trusted content.

## Activation state

The agent lives outside the managed application release tree. Local state tracks at least current, previous known-good, and pending release identities plus accepted channel/trust state.

A typical lifecycle is:

```text
stage -> activate pointer -> launch -> application confirms -> keep
                               │
                               └─ timeout/crash -> recover previous known-good
```

This avoids pretending that replacing a non-empty running Windows installation directory is universally atomic.

## Security baseline

- TLS for all network paths.
- Separate publisher credentials from anonymous/authenticated client download policy.
- SHA-256 content verification is mandatory.
- Canonical JSON is the only digest/signing input for canonical JSON resources.
- Path validation rejects cross-platform unsafe materialization semantics.
- Release descriptors/manifests/artifacts become immutable after commit.
- Digest verification provides integrity, not publisher authenticity or freshness.
- Production trust metadata/signing is a separate security layer and should follow a rollback/freeze-resistant update framework rather than an ad-hoc signature field.

## Initial delivery slices

1. RSP foundation: release descriptor, channel revision, canonical JSON/digest, paths, HTTP resource semantics and fixtures.
2. Local filesystem CAS and metadata persistence.
3. `reship publish` transaction and compare-and-swap channel promotion.
4. Read-only RSP HTTP API with ETag and range-capable artifacts.
5. Rust client core/agent and local client API.
6. Thin .NET SDK integration.
7. Windows activation/bootstrap recovery flow.
8. Production trust/signing metadata.
9. S3-compatible origin + CDN/edge deployment.
10. Metrics/telemetry and policy layers.

Each slice should be tracked as bounded `RS-*` work in YouTrack.
