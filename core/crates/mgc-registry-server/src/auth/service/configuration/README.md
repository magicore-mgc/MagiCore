# configuration

Authentication and trusted transport configuration.

Cấu hình admin/trusted publishing, proxy, loopback và key.

`mod.rs` declares the owner boundary and `configuration.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
