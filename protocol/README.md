# RSP — ReShip Protocol

RSP is the application-neutral protocol used by ReShip. It is versioned independently from the ReShip server, CLI, storage implementation, and SDKs.

Dependency direction is one-way: **ReShip depends on RSP; RSP never depends on ReShip**. RSP must not contain database, object-storage, CDN, region-routing, authentication-provider, scheduler, or application-specific concepts.

The protocol is decomposed into focused crates:

- `rsp-core` — stable identifiers, protocol version, content digests, target triples, and release ordering primitives.
- `rsp-manifest` — immutable installation snapshots. A manifest describes exactly which files belong to a release; absence means deletion when constructing the next clean snapshot.
- `rsp-discovery` — channel state and release selection semantics, independent of transport.
- `rsp-http` — the HTTP binding for RSP, including media types and CDN-friendly resource paths. HTTP is a binding, not the semantic protocol itself.

RSP v1 intentionally specifies file-level differential delivery rather than binary patches. Clients compare the installed manifest to the target manifest and derive `keep`, `download`, and `delete` operations. Binary delta resources may be added later without changing the snapshot model.
