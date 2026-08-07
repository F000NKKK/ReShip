# CI and test rules

There are two independent Rust workspaces: root ReShip (`crates/*`) and `protocol/Cargo.toml` RSP. Run fmt, clippy `-D warnings`, tests and rustdoc for every affected workspace. RSP changes always require protocol checks even if ReShip code is untouched.

Tests are deterministic/non-destructive by default. Platform activation/bootstrapper tests use isolated temporary installations and explicit integration/native CI. Preserve the CI architecture check forbidding RSP manifest dependencies on `reship-*`.

When platform-specific SDK/bootstrapper code exists, test supported native OS/runtime combinations. Never weaken verification merely to obtain a green run.
