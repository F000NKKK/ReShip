# Contributing to ReShip

ReShip is pre-1.0 and its protocol boundaries are deliberately conservative even while implementation APIs evolve.

Before contributing:

1. Read `README.md`, `index.md`, and `ARCHITECTURE.md`.
2. For maintainer/agent work, identify the governing YouTrack `RS-*` item and any ADR.
3. Keep changes bounded. Do not combine protocol redesign, server implementation, SDK work and unrelated cleanup in one patch.
4. Preserve the independent `/protocol` workspace and the one-way ReShip/SDK -> RSP dependency direction.

Required verification for affected workspaces is fmt, clippy with warnings denied, tests and rustdoc. CI runs both workspaces on Linux, Windows and macOS.

Protocol/wire changes require an ADR and explicit compatibility analysis. Published releases and content-addressed resources are immutable by design.

External contributors who do not have access to the project tracker may open a GitHub issue/PR with the problem and proposed scope; maintainers will associate it with ReShip tracking before merge when needed.
