# Versioning rules

ReShip implementation and RSP are independently versioned. `reship-*` versions describe implementation packages; `rsp-*` versions describe protocol/API crates.

Application version strings are display metadata; release ordering uses `ReleaseSequence`. Published releases/CAS resources are immutable.

Pre-1.0 Rust API breakage may occur with an ADR and synchronized consumers, but RSP wire semantics are stricter: never silently reinterpret an existing protocol field. Breaking semantic/wire changes require an explicit protocol-version decision and ADR. Additive features must preserve old-client behavior or define negotiation.
