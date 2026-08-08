# ReShip SDKs

SDKs are consumers of RSP, not extensions of the ReShip server implementation. The security-critical updater state machine should be implemented once by the Rust ReShip client/agent layer and exposed to language SDKs through a versioned local client API.

The shared client state machine is:

1. conditionally poll channel state (`ETag`/`If-None-Match` when using the HTTP binding);
2. reject stale channel revisions while allowing deliberate rollback represented by a newer `ChannelRevision`;
3. compare the installed release identity with the desired immutable release descriptor;
4. fetch the release descriptor and select the manifest for the local target;
5. fetch and verify the immutable target manifest;
6. determine which content-addressed artifacts are already present in the verified local CAS and download only missing content from ordered delivery endpoints with fallback;
7. verify every downloaded artifact digest before admitting it to the local CAS;
8. materialize a clean staged snapshot exclusively from verified CAS content, then delegate activation/restart/rollback to the platform bootstrapper;
9. persist installed state only after successful activation confirmation.

A manifest diff may expose `keep / download / delete` for diagnostics and planning, but activation does not mutate the active installation file-by-file. Files absent from the target manifest simply do not appear in the newly materialized snapshot.

The first language integration target is .NET. Language SDKs should remain thin integration surfaces and must not independently reimplement RSP trust, download, CAS, staging, or rollback semantics.
