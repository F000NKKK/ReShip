# CI and test rules

ReShip has two Rust workspaces and both are verification units:

- root workspace: ReShip implementation (`crates/*`);
- `protocol/Cargo.toml`: independent RSP workspace.

For affected work run formatting, clippy with `-D warnings`, tests, and rustdoc with warnings denied. Protocol changes must run the protocol workspace even when no ReShip crate changed.

Ordinary tests must be deterministic, non-privileged and non-destructive. Platform bootstrapper/integration tests that mutate real installations must use isolated temporary roots and explicit opt-in/CI jobs.

CI must preserve the architecture boundary check that RSP manifests do not depend on `reship-*` crates.

When SDK/bootstrapper implementations ship, verify their supported OS/runtime matrix on native CI where platform behavior matters. A Linux-only pass does not establish Windows activation correctness.

Never relax lint/test/integrity policy merely to make CI green.
