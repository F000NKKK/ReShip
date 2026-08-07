# ReShip repository map

Read this file before making repository changes.

## Dependency direction

```text
protocol/RSP  <── sdk/*
     ↑
     ├──────── crates/reship-server
     ├──────── crates/reship-cli
     └──────── crates/reship-storage
```

Never add a `reship-*` dependency to any `protocol/rsp-*` manifest.

## Workspaces

- `/Cargo.toml` — ReShip implementation workspace (`crates/*`).
- `/protocol/Cargo.toml` — independent RSP workspace (`protocol/rsp-*`).

Both workspaces have their own build/test/doc verification and may evolve/version independently.

## Current implementation

- `rsp-core`: IDs, digest, protocol version, release sequence.
- `rsp-manifest`: immutable release snapshot model.
- `rsp-discovery`: channel desired state.
- `rsp-http`: initial HTTP/CDN binding.
- `reship-storage`: storage boundary placeholder.
- `reship-server`: minimal health-serving process; no release API yet.
- `reship-cli`: command surface placeholders only.
- `sdk/README.md`: language SDK behavior contract.

Do not infer that a documented future slice is implemented. `README.md`, `ARCHITECTURE.md`, and `CHANGELOG.md` must distinguish current behavior from planned behavior.
