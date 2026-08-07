# ReShip SDKs

SDKs are consumers of RSP, not extensions of the ReShip server implementation. Each language SDK must implement the same state machine:

1. conditionally poll channel state (ETag/If-None-Match when using the HTTP binding);
2. compare the installed release sequence to channel state;
3. fetch and verify the immutable target manifest;
4. derive a local `keep / download / delete` plan by comparing manifests;
5. download missing content-addressed artifacts from ordered delivery endpoints with origin fallback;
6. verify every artifact digest before staging;
7. construct a clean staged snapshot, then delegate process/file replacement to a platform bootstrapper;
8. persist the installed manifest only after successful activation.

The first integration target is .NET for ETS Client. Rust is the reference protocol implementation. Additional SDKs must not require server-side changes.
