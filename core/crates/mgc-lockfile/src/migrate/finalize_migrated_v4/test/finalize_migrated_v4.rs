//! Confirms lossy package data is refused by finalization.
//! Xác nhận dữ liệu package hao hụt bị finalizer từ chối.

use super::super::finalize_migrated_v4;
use crate::{Lockfile, Package};

#[test]
fn refuses_a_package_without_install_integrity() {
    let mut lock = Lockfile::new();
    let mut package = Package::new(
        "sample".into(),
        "1.0.0".into(),
        "https://example.invalid/sample.tgz".into(),
        String::new(),
    );
    package.registry = Some("https://registry.example".to_string());
    lock.packages.push(package);
    let error = finalize_migrated_v4(lock, "mgc/test", "2026-10-17T00:00:00Z")
        .expect_err("test operation must fail");
    assert!(
        error
            .warnings
            .iter()
            .any(|warning| warning.contains("no integrity"))
    );
}
