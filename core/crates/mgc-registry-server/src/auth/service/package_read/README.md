# package_read

Package read and scope authorization.

Gate package/global read, scope map và quyền publish.

`mod.rs` declares the owner boundary and `package_read.rs` contains its implementation or model re-exports. The existing shared regression test stays at `src/test/auth.rs` during the source-only pass; test splitting/movement follows after repository-wide production-source decomposition.
