# trusted_package_write

Trusted package write authorization.

Kiểm tra package, peer transport, repository binding và generation trước write.

`mod.rs` declares the owner boundary and `trusted_package_write.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
