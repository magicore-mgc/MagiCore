# user_role

Registry role model.

Role viewer/publisher/admin cùng quy tắc parse/publish.

`mod.rs` declares the owner boundary and `user_role.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
