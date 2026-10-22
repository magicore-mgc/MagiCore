# auth_service

Authentication service state.

State AuthService; field nội bộ chỉ mở trong module auth.

`mod.rs` declares the owner boundary and `auth_service.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
