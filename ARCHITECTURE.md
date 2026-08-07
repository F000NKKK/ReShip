# ReShip architecture

**Languages:** English | [Русский](ARCHITECTURE.ru.md)

## v0.1 goal

Establish a small, reliable release-distribution platform that can replace application-specific update chains without recreating them as a fleet of microservices.

ReShip v0.1 has three architectural planes:

1. **RSP protocol plane** — independently versioned semantics and bindings.
2. **Control plane** — publishing, applications, channels, release metadata, policy, and delivery selection.
3. **Data plane** — immutable manifests/artifacts delivered from origin, CDN, or regional mirrors.

The updater/SDK is a client of those planes, not part of the control plane.

## Hard invariants

1. Releases are immutable after publication.
2. A release manifest is a complete installation snapshot, not a list of copied-over files.
3. Artifacts are content-addressed and verified before activation.
4. Channel state is mutable; release content is immutable.
5. Update availability is durable state, never a one-shot notification event.
6. RSP is application-neutral and infrastructure-neutral.
7. RSP has no dependency on ReShip implementation crates.
8. HTTP is an RSP binding, not RSP itself.
9. CDN is optional. The same client must work against origin only.
10. Regional scaling changes delivery endpoints, not manifest semantics.
11. CI publishes; it is not part of the runtime update path.
12. Activation must construct a clean target snapshot and provide rollback boundaries.

## RSP decomposition

`protocol/` is a nested Cargo workspace with its own package metadata and version cadence.

```text
rsp-core
   ↑
   ├──── rsp-manifest
   └──── rsp-discovery
            ↑
            └──── rsp-http
```

`rsp-core` owns protocol version, IDs, digests, release sequence and target selectors.

`rsp-manifest` owns the immutable filesystem snapshot. It does not prescribe storage URLs.

`rsp-discovery` owns the desired release for an application/channel/target stream, including minimum-supported sequence.

`rsp-http` maps semantic resources to HTTP media types, conditional channel-state polling, immutable CAS paths, and ordered delivery endpoints. A future gRPC or filesystem binding must not require changes to semantic crates.

## Release identity and ordering

Human version strings are display metadata and are not trusted for ordering. Every application/channel/target stream has a monotonic `ReleaseSequence`. A publisher creates a unique immutable `ReleaseId` and assigns the next sequence when the release becomes eligible for that stream.

This avoids hard-coding SemVer into RSP and allows legacy applications with existing version formats.

## Snapshot and differential update

A manifest describes every managed file in the target installation tree. Comparing installed and target manifests yields:

```text
same path + same digest   -> KEEP
same/new path + new hash -> DOWNLOAD
old path absent in target -> DELETE / do not copy to staging
```

The preferred implementation constructs a new staging tree from the target manifest and atomically swaps it into place. It does not mutate the current installation file-by-file.

Binary patches are explicitly out of scope for the first protocol version. They can later become an optional artifact representation while the target manifest remains authoritative.

## Control plane

The first server is one deployable Rust service, not a microservice fleet. Logical modules may later be split only with operational evidence.

Responsibilities:

- register applications and channels;
- accept authenticated release publication;
- validate paths, uniqueness and digests;
- commit immutable release metadata;
- atomically move channel pointers;
- expose RSP discovery resources;
- choose ordered delivery endpoints based on configuration/region;
- expose health/metrics and audit events.

A relational metadata database and artifact storage are implementation details behind boundaries. v0.1 may start with one metadata store and filesystem/S3-compatible origin.

## Data plane and CDN

Immutable resources use content-addressed paths:

```text
/rsp/v1/manifests/<algorithm>/<digest>
/rsp/v1/artifacts/<algorithm>/<digest>
```

They are safe for long CDN cache lifetimes because content at a digest URL never changes.

Channel state is different:

```text
/rsp/v1/apps/<app>/channels/<channel>/state
```

It is mutable, small, and uses ETag/`If-None-Match` with short or no intermediary caching according to deployment policy.

Without CDN, delivery endpoints point at the ReShip origin or an object-store gateway. With CDN, the first endpoint points at an edge hostname and origin remains a fallback. Clients always verify the digest, so a stale/corrupt edge object cannot become a valid release.

## Multi-region

v0.1 does not require active-active metadata writes. The recommended starting topology is one authoritative control plane plus globally/regionally replicated immutable data.

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

Later regional control-plane replicas may serve read-only discovery from replicated channel state. Publication remains serialized until a concrete need justifies distributed write coordination.

## SDK contract

An SDK performs discovery, manifest verification, update-plan derivation, artifact download/fallback, staging and persistence of installed state. Process replacement is delegated to a platform bootstrapper because an application cannot safely replace its own executable/files while running.

The first production SDK target is .NET for ETS Client. RSP remains language-neutral.

## Security baseline

- TLS for all network paths.
- Separate publisher credentials from anonymous/authenticated client download policy.
- SHA-256 content verification is mandatory in v0.1.
- Path validation rejects absolute paths, traversal and duplicate normalized paths.
- Manifests/releases become immutable after commit.
- Future release signing must be additive to RSP; digest verification is not a substitute for publisher authenticity.

## Initial delivery slices

1. RSP validation + canonical JSON rules and fixtures.
2. Local filesystem CAS and metadata persistence.
3. `reship publish` transaction and channel promotion.
4. Read-only RSP HTTP API with ETag and range-capable artifacts.
5. .NET SDK check/download/update-plan implementation.
6. Windows bootstrapper with staging, atomic swap/rollback and restart.
7. ETS Test integration.
8. S3-compatible origin + CDN deployment guidance.
9. Metrics/telemetry and minimum-supported-version policy.

Each slice should be tracked as bounded `RS-*` work in YouTrack.
