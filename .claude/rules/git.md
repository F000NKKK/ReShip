# Git rules

- Preserve unrelated changes and inspect `git status`/`git diff` around bounded edits.
- Never reset hard, clean, broad-checkout, amend, rebase or force-push. Mechanical deny rules are in `.claude/settings.json`.
- Create commits only when authorized.
- Normal commits use `<type>/RS-<id>: <summary>` with Conventional Commit type; mention additional `RS-*` and ADR Article IDs in the body.
- The repository bootstrap predating the first ReShip YouTrack issue is the only exception to the `RS-*` commit requirement.
- Avoid auto-transition keywords unless explicitly required by the active workflow.
- Do not reuse merged task branches; merged same-repo branches are expected to be deleted automatically.
