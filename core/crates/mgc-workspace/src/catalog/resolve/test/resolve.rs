//! Default, named and unsupported catalog specifier tests.
//! Test catalog mặc định, có tên và specifier không hỗ trợ.

use crate::catalog::{WorkspaceCatalogs, resolve_catalog_specifier};
use std::collections::HashMap;

#[test]
fn resolves_default_and_named_catalogs() {
    let mut catalogs = WorkspaceCatalogs::default();
    catalogs
        .default
        .insert("serde".to_string(), "1.0".to_string());
    catalogs.named.insert(
        "tools".to_string(),
        HashMap::from([("cargo-nextest".to_string(), "0.9".to_string())]),
    );

    assert_eq!(
        resolve_catalog_specifier("serde", "catalog:", &catalogs),
        Some("1.0".to_string())
    );
    assert_eq!(
        resolve_catalog_specifier("serde", "catalog:default", &catalogs),
        Some("1.0".to_string())
    );
    assert_eq!(
        resolve_catalog_specifier("cargo-nextest", "catalog:tools", &catalogs),
        Some("0.9".to_string())
    );
}

#[test]
fn non_catalog_specifier_does_not_resolve() {
    let catalogs = WorkspaceCatalogs::default();

    assert_eq!(resolve_catalog_specifier("serde", "1.0", &catalogs), None);
}

#[test]
fn missing_default_and_named_entries_do_not_resolve() {
    let mut catalogs = WorkspaceCatalogs::default();
    catalogs
        .named
        .insert("tools".to_string(), std::collections::HashMap::new());

    assert_eq!(
        resolve_catalog_specifier("missing", "catalog:", &catalogs),
        None
    );
    assert_eq!(
        resolve_catalog_specifier("missing", "catalog:unknown", &catalogs),
        None
    );
    assert_eq!(
        resolve_catalog_specifier("missing", "catalog:tools", &catalogs),
        None
    );
}
