# authentication

Required authentication middleware.

Giữ route public hiện có, từ chối malformed header và gắn user xác thực.

`mod.rs` declares the owner boundary and `authentication.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
