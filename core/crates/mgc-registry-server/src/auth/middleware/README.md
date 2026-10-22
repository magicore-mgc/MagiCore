# middleware

Authentication middleware entry points.

Challenge 401 cùng middleware auth theo request.

`mod.rs` declares the owner boundary and `middleware.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
