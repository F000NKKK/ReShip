# ReShip Claude Code configuration

This directory is the native Claude Code equivalent of `.codex/`. It is self-contained at runtime and follows the same policy: YouTrack project **ReShip** (`RS`), role pipeline, Git safety, RSP boundary, CI and versioning rules.

`CLAUDE.md` is the root entry point and imports every rule. Bounded work is dispatched through `researcher`, `architect`, `implementer`, and `reviewer` definitions in `.claude/agents/`.

Keep `.claude/` and `.codex/` semantically synchronized when workflow rules change. Claude uses `mcp__youtrack__*` for issue access; Codex follows its YouTrack REST rule.
