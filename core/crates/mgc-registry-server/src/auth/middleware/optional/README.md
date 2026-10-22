# optional

Optional authentication middleware.

Gắn user khi Bearer hợp lệ, còn lại tiếp tục request.

`mod.rs` declares the owner boundary and `optional.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
