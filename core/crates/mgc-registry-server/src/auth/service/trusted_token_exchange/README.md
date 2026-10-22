# trusted_token_exchange

OIDC trusted token exchange.

Validate OIDC/Sigstore identity, repository binding, quota và TTL trước khi cấp token.

`mod.rs` declares the owner boundary and `trusted_token_exchange.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
