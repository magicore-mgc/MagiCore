# account_admin

Registry account administration.

Yêu cầu admin đã cấu hình trước khi sửa tài khoản.

`mod.rs` declares the owner boundary and `account_admin.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
