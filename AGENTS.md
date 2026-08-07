# ReShip agent workflow

This is the repository entry point for Codex and collaborating agents.

Before changing anything, read:

1. `index.md` for workspace/dependency boundaries;
2. `ARCHITECTURE.md` and `ARCHITECTURE.ru.md` for current invariants and roadmap;
3. `.codex/README.md`, all `.codex/rules/*.md`, and the selected role profile;
4. the active YouTrack work item in project **ReShip** (`RS`) and its parent hierarchy/comments/linked ADR Articles.

All roadmap work is tracked with IDs `RS-*`. Do not invent an issue ID. Repository-bootstrap work predating creation of the first ReShip YouTrack task is the only allowed exception; all subsequent implementation commits require an `RS-*` ID.

## Role pipeline

```text
researcher
    ↓ evidence
architect
    ↓ design / ADR when needed
implementer
    ↓ source + tests + docs
reviewer
    ↓ independent verification
primary agent
    ↓ YouTrack reconciliation
```

Each role works one bounded Task/Story. Do not silently expand scope. Public protocol or cross-workspace decisions require an ADR before implementation.

## ReShip-specific invariant

`protocol/` is RSP, an independently versioned nested workspace. Dependency direction is strictly **ReShip/SDK -> RSP**. No `protocol/rsp-*` crate may depend on `reship-*`, storage backends, database clients, CDN vendors, region routing, or application-specific code.

RSP semantic crates must remain transport-neutral; transport details belong in binding crates such as `rsp-http`.

## Completion

A task is complete only when implementation, deterministic tests, rustdoc, English/Russian documentation, package metadata, CI expectations, and YouTrack evidence agree. Preserve unrelated changes and never use destructive Git operations.
