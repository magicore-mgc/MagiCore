# clone

Authentication service clone semantics.

Clone state/cache theo semantics hiện có.

`mod.rs` declares the owner boundary and `clone.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
