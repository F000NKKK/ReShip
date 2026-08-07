# Git rules

- Inspect status/diff before and after bounded changes.
- Preserve unrelated work.
- Never use `git reset --hard`, `git clean`, broad checkout, amend, rebase, or force-push.
- Do not create commits unless the user/task authorizes it.
- Every normal work commit must use:

  `<type>/RS-<id>: <summary>`

  where type is an appropriate Conventional Commits type (`feat`, `fix`, `docs`, `refactor`, `chore`, `perf`, `test`, `ci`, `build`). Mention additional `RS-*` IDs and applicable ADR Article IDs in the body.
- Do not use auto-transition keywords such as `fixes RS-*` unless the active workflow explicitly requires them.
- The initial repository bootstrap predating the first ReShip YouTrack issue is the sole exception to the `RS-*` commit rule; never extend that exception to future work.
- Keep protocol, implementation, SDK, tests, and documentation patches reviewable.
- A merged same-repository branch is expected to be deleted by repository workflow; do not reuse merged task branches.
