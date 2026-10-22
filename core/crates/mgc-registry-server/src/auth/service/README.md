# service

Authentication service construction.

Khởi tạo AuthService và sở hữu các operation service trong module con.

`mod.rs` declares the owner boundary and `service.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
