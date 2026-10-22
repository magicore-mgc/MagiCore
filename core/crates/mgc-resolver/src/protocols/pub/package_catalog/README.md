# Package catalog / Catalog package

`types.rs` owns serde-only response DTOs. `package_catalog.rs` selects the highest compatible version and enforces archive URL/SHA-256 plus SDK environment rules. `dependency_metadata/` parses PubSpec dependency values; `package_name/` validates URL path identity. `error.rs` maps typed catalog failures to established `MgError` categories/messages. `mod.rs` is wiring and narrow internal exports.

Selection errors preserve `DependencyConflict`, `Other` and `Integrity`; invalid metadata and non-registry sources remain fail-closed. Network behavior is covered by `core/crates/mgc-resolver/tests/protocols_pub.rs`.
