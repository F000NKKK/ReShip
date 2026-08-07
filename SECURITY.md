# Security policy

ReShip distributes executable application content, so integrity and update activation are security boundaries.

## Reporting

Do not publish exploitable vulnerabilities in a public issue. Prefer GitHub private vulnerability reporting / security advisories for this repository when available. If that channel is unavailable, contact the repository owner privately through an established project contact before disclosure.

Include affected component/version, impact, prerequisites, reproduction details, and any evidence about manifest/artifact verification or activation behavior.

## Baseline guarantees targeted by v0.1

- TLS on network delivery paths.
- Mandatory cryptographic content-digest verification before staging/activation.
- Strict relative-path normalization; traversal and duplicate normalized paths are rejected.
- Immutable committed release metadata and content-addressed resources.
- Publisher authorization is separate from client download policy.
- Clients do not trust CDN/origin bytes merely because transport succeeded.

Release signing/authentic publisher identity is planned as an additive trust layer; SHA-256 content integrity alone does not authenticate the publisher.
