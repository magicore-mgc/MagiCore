# user

Registry user model.

User và Debug formatter che password.

`mod.rs` declares the owner boundary and `user.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
