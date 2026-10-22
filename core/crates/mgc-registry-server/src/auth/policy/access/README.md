# access

Public access policy wrappers.

Giữ API check_scope_access/check_publish_access theo AuthService.

`mod.rs` declares the owner boundary and `access.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
