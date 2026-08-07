# Claude Code entry point

ReShip's native Claude Code workflow lives under `.claude/`. Read before making changes:

1. `index.md`.
2. `ARCHITECTURE.md` and `ARCHITECTURE.ru.md`.
3. `.claude/README.md`, imported rules below, and the matching `.claude/agents/*.md` role.
4. The active YouTrack project **ReShip** (`RS`) issue via `mcp__youtrack__*`; read parent issues, comments, and linked ADR Articles before editing.

All normal work uses `RS-*` IDs. Never invent one. The initial repository bootstrap before the first ReShip YouTrack issue is the only exception.

## Role pipeline

Dispatch bounded work in this order:

```text
researcher -> architect -> implementer -> reviewer
```

The primary agent reconciles each handoff with YouTrack and advances workflow state only when the evidence required by the YouTrack rule permits it.

@.claude/rules/youtrack.md
@.claude/rules/ci.md
@.claude/rules/research.md
@.claude/rules/files.md
@.claude/rules/git.md
@.claude/rules/versioning.md

## Hard architecture rule

`protocol/` is the independent RSP workspace. RSP may not depend on any `reship-*` crate or ReShip infrastructure concern. HTTP belongs to `rsp-http`; semantic protocol crates remain transport-neutral.

The destructive Git restrictions in `.claude/rules/git.md` are also enforced through `.claude/settings.json`.
