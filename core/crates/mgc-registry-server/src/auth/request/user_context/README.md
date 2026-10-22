# user_context

Request user context.

Đọc User đã gắn vào Axum request extensions.

`mod.rs` declares the owner boundary and `user_context.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
