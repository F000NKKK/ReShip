# File and documentation rules

Read `CLAUDE.md`, `index.md`, architecture docs and active `RS-*` context before editing.

Keep `/protocol`, `/crates`, `/sdk` boundaries. RSP semantic crates cannot contain ReShip server/storage/database/CDN/region/application details. HTTP belongs in `rsp-http` or another binding crate, not semantic crates.

Public Rust APIs need rustdoc. Synchronize affected English/Russian README/architecture docs. Review changelog/security/support/contributing/manifests/CI when policy or behavior changes. Do not commit generated build output, secrets, or local environment files.
