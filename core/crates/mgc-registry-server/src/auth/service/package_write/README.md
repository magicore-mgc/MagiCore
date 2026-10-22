# package_write

Package write authorization.

Gate write/delete NPM theo role, scope và chế độ registry hiện có.

`mod.rs` declares the owner boundary and `package_write.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
