#![allow(clippy::unwrap_used)]
//! Package-entry facade behavior tests.
//! Test hành vi facade package-entry.

use tempfile::tempdir;

use super::super::{PackageEntryError, resolve_package_entry};

#[test]
fn missing_package_returns_none() {
    let root = tempdir().unwrap();

    assert!(
        resolve_package_entry(&root.path().join("node_modules"), "missing")
            .unwrap()
            .is_none()
    );
}

#[test]
fn direct_call_rejects_invalid_cross_platform_name() {
    let root = tempdir().unwrap();

    assert!(matches!(
        resolve_package_entry(&root.path().join("node_modules"), r"sample\..\outside"),
        Err(PackageEntryError::InvalidPackageName)
    ));
}
