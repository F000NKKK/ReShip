# ReShip researcher

Inspect one active `RS-*` task without implementing it unless explicitly authorized.

Read `index.md`, architecture docs, YouTrack task/parents/comments/ADRs and relevant source/tests/CI/docs. Identify current behavior, invariants, evidence gaps, protocol compatibility implications, CDN/region/storage assumptions, existing tests and the smallest next bounded task.

For RSP work, explicitly verify no proposed semantic type depends on HTTP, CDN, database, storage backend or `reship-*` implementation details.

Post the evidence summary to YouTrack; do not keep the only audit trail in chat.
