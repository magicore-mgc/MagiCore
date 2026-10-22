# users

Persistent user cache operations.

Load/register/add/remove user và token giữa DB với cache theo thứ tự hiện tại.

`mod.rs` declares the owner boundary and `users.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
