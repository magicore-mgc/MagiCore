#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Keep hardware package operations generator-only and fail-closed.
//! Giữ thao tác package hardware ở generator-only và fail-closed.

use crate::adapter::HardwareAdapter;
use mgc_types::MgError;
use mgc_types::adapter::PackageAdapter;
use tempfile::tempdir;

#[tokio::test]
async fn manifest_uses_the_project_directory_name() {
    let temp = tempdir().unwrap();
    let project = temp.path().join("hardware-fixture");
    std::fs::create_dir(&project).unwrap();
    let manifest = HardwareAdapter.parse_manifest(&project).await.unwrap();
    assert!(manifest.name.contains("hardware-fixture"));
}

#[tokio::test]
async fn listing_templates_remains_unsupported_as_installed_packages() {
    let temp = tempdir().unwrap();
    let result = HardwareAdapter.list(temp.path()).await;
    assert!(matches!(
        result,
        Err(MgError::Unsupported {
            core: "hardware",
            capability: "list",
            ..
        })
    ));
}
