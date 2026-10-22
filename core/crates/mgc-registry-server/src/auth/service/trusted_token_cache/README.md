# trusted_token_cache

Trusted token cache lifecycle.

Prune token hết hạn, lookup identity/binding và revoke token.

`mod.rs` declares the owner boundary and `trusted_token_cache.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
