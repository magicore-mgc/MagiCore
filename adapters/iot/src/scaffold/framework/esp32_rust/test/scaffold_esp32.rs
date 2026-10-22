#![cfg(test)]
#![allow(clippy::unwrap_used)]
//! Preserve the existing ESP32 Rust scaffold output contract.
//! Giữ contract output scaffold ESP32 Rust hiện có.

use crate::framework::IotFramework;
use crate::scaffold::scaffold_project;
use crate::scaffold::test::support::tmp;

#[tokio::test]
async fn test_scaffold_esp32() {
    let tmp = tmp();
    scaffold_project(IotFramework::Esp32Rust, "test", "esp32c3", tmp.path())
        .await
        .unwrap();
    assert!(tmp.path().join("Cargo.toml").exists());
    assert!(tmp.path().join("mgc.toml").exists());
}

#[tokio::test]
async fn invalid_cargo_name_is_rejected_before_project_creation() {
    let temp = tmp();
    let project = temp.path().join("firmware");
    let project_name = "device\"\n[injected]\nowned = \"yes";
    let error = scaffold_project(IotFramework::Esp32Rust, project_name, "esp32c3", &project)
        .await
        .expect_err("invalid Cargo names must be rejected");

    let diagnostic = error.to_string();
    assert!(matches!(error, mgc_types::MgError::InvalidPackageName(_)));
    assert!(
        !diagnostic.contains(project_name),
        "diagnostic must not echo raw input"
    );
    assert!(
        !project.exists(),
        "validation must precede filesystem writes"
    );
}
