# Versioning rules

ReShip implementation and RSP are independently versioned. `reship-*` versions describe implementation packages; `rsp-*` versions describe protocol/API crates.

Application version strings are display metadata and do not order releases. Immutable releases have identity; mutable channel transitions are ordered by monotonic `ChannelRevision`, which advances for both promotion and rollback. A channel points to an immutable release descriptor, and promotion must not rebuild or mutate that descriptor, its manifests, or artifacts.

Pre-1.0 Rust API breakage may occur with an ADR and synchronized consumers, but RSP wire semantics are stricter: never silently reinterpret an existing protocol field. Breaking semantic/wire changes require an explicit protocol-version decision and ADR. Additive features must preserve old-client behavior or define negotiation.
