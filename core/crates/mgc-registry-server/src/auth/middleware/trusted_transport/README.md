# trusted_transport

Trusted token transport middleware.

Chặn token trusted trên transport chưa xác thực.

`mod.rs` declares the owner boundary and `trusted_transport.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
