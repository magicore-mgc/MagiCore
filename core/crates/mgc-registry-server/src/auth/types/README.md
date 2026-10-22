# types

Authentication and identity models.

Mục lục model auth, user, role và trusted token.

`mod.rs` declares the owner boundary and `types.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
