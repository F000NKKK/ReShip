# Versioning rules

ReShip implementation and RSP are independently versioned.

- `reship-*` versions describe implementation packages/binaries.
- `rsp-*` versions describe protocol/API crates and may evolve on a different cadence.
- A ReShip release must declare which RSP protocol major/minor it serves.
- Human application `ReleaseVersion` strings are not used for ordering; RSP ordering uses `ReleaseSequence`.
- Before 1.0, breaking Rust APIs may evolve with an ADR and synchronized consumers, but wire-format compatibility is still explicit: do not silently reinterpret an existing RSP v1 field.
- Additive optional fields/representations must preserve old-client behavior or require an explicit protocol-version negotiation rule.
- A breaking semantic/wire change requires a deliberate RSP protocol-version decision and ADR, even while crates remain pre-1.0.
- Release manifests and content-addressed resources are immutable once published; version bumps never rewrite old resources.
