# File and documentation rules

- Read `AGENTS.md`, `index.md`, architecture docs, and the active `RS-*` issue before edits.
- Preserve the split between `/protocol`, `/crates`, and `/sdk`.
- Never put ReShip server/storage/CDN/database semantics into RSP semantic crates.
- New transport semantics belong in a dedicated RSP binding crate; do not leak HTTP concepts into `rsp-core`, `rsp-manifest`, or `rsp-discovery`.
- Public Rust items require rustdoc.
- Synchronize English/Russian README and architecture content when a shared concept changes.
- Review `CHANGELOG.md`, `SECURITY.md`, `SUPPORT.md`, `CONTRIBUTING.md`, manifests and CI when behavior/policy changes.
- Public docs must stand alone; do not require private YouTrack access to understand product behavior. Do not put `RS-*`/ADR IDs in user-facing API docs as the only explanation of a decision.
- Never edit generated `target/` content or commit local secrets/environment files.
