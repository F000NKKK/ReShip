# ReShip Codex configuration

Durable repository workflow for Codex. Task-specific scope, evidence and decisions live in the YouTrack project **ReShip** (`RS`), not in `.ai/` files.

Load order:

1. root `AGENTS.md`, `index.md`, architecture documents;
2. active `RS-*` issue, parent hierarchy, comments, and linked ADR Articles;
3. all rule files in `.codex/rules/`;
4. the role profile in `.codex/agents/`.

Role order is researcher -> architect -> implementer -> reviewer. Every handoff records inspected files/contracts, evidence, commands, unresolved risks and next role on the active YouTrack issue.

RSP is a separate nested workspace. Any change crossing the RSP/ReShip boundary requires explicit architect review and normally an ADR.
