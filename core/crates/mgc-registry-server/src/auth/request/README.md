# request

Authentication request helpers.

Helper request context và parser header.

`mod.rs` declares the owner boundary and `request.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
