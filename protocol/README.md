# RSP — ReShip Protocol

RSP is the application-neutral protocol used by ReShip. It is versioned independently from the ReShip server, CLI, storage implementation, and language SDKs.

Dependency direction is one-way: **ReShip depends on RSP; RSP never depends on ReShip**. RSP must not contain database, object-storage, CDN, region-routing, authentication-provider, scheduler, or application-specific concepts.

The protocol is decomposed into focused crates:

- `rsp-core` — protocol version, stable identifiers, canonical content digests, channel revisions, content lengths, and target identifiers.
- `rsp-manifest` — immutable installation snapshots for one target. A manifest describes exactly which regular files belong to the target snapshot.
- `rsp-release` — immutable logical release descriptors that map one application release to one or more target manifests.
- `rsp-discovery` — mutable channel state. A channel points to an immutable release descriptor and has a monotonic `ChannelRevision` that advances for both promotion and rollback.
- `rsp-json` — deterministic canonical JSON bytes used for stable hashing/signing inputs and wire fixtures.
- `rsp-http` — the HTTP binding for RSP, including media types, cache policy constants, and resource paths. HTTP is a binding, not the semantic protocol itself.

The core resource graph is:

```text
ChannelState (mutable)
    -> ReleaseDescriptor (immutable)
        -> ReleaseManifest per TargetId (immutable)
            -> artifacts by ContentDigest (immutable)
```

Human-facing release versions are metadata only and do not order releases. Immutable releases have identity; only channel mutations are ordered, using `ChannelRevision`. Promoting the same release between channels must not rebuild or mutate its descriptor, manifests, or artifacts.

RSP v1 uses a conservative portable manifest-path grammar and content-addressed immutable resources. Large 64-bit protocol values are encoded as canonical decimal strings to avoid precision loss in JSON consumers. SHA-256 digests use the canonical `sha256:<64 lowercase hex>` form.

RSP v1 intentionally specifies file-level differential network delivery rather than binary patches. A client/agent may compare snapshots for diagnostics and download planning, but activation materializes a new clean snapshot from verified content rather than deleting/copying files in-place. Binary delta resources may be added later without changing the authoritative snapshot model.
