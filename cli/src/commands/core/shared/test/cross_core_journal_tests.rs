//! Test that commands from other cores reject foreign journals.
//! Kiểm tra command core khác từ chối journal không thuộc nó.

use super::*;

#[cfg(feature = "game")]
#[tokio::test]
async fn game_bevy_journal_rejected_by_godot_command() {
    // Blocker 1: cùng adapter game, khác engine — bevy journal qua tay
    // godot command phải fail (core giống nhau chưa đủ).
    // (Same adapter, different engine — must mismatch.)
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("project.godot"), "; godot\n").unwrap();
    let adapter =
        mgc_game_adapter::adapter_for(root).expect("project.godot must detect a game adapter");
    let write_lock = ProjectWriteLock::acquire(root, std::time::Duration::from_secs(30)).unwrap();
    craft_identity_journal(root, "game", "bevy", "Cargo.toml", "Cargo.toml");
    let err = recover_interrupted_remove(&adapter, root, &write_lock)
        .await
        .expect_err("bevy journal via godot command must fail closed");
    assert!(
        format!("{err:#}").contains("bevy"),
        "must name the staged lane: {err:#}"
    );
    drop(write_lock);
}

#[cfg(feature = "iot")]
#[tokio::test]
async fn iot_esp32_journal_rejected_by_platformio_command() {
    // Blocker 1: cùng adapter iot, khác framework — esp32 journal qua
    // tay platformio command phải fail.
    // (Same adapter, different framework — must mismatch.)
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("platformio.ini"), "[env:test]\n").unwrap();
    let adapter =
        mgc_iot_adapter::adapter_for(root).expect("platformio must detect an iot adapter");
    let write_lock = ProjectWriteLock::acquire(root, std::time::Duration::from_secs(30)).unwrap();
    craft_identity_journal(root, "iot", "esp32-rust", "Cargo.toml", "Cargo.toml");
    recover_interrupted_remove(&adapter, root, &write_lock)
        .await
        .expect_err("esp32 journal via pio command must fail closed");
    drop(write_lock);
}

#[cfg(feature = "clo")]
#[tokio::test]
async fn cloud_terraform_journal_rejected_by_cdk_command() {
    // Blocker 1: cùng adapter cloud, khác type — terraform journal qua
    // tay cdk command phải fail.
    // (Same adapter, different cloud type — must mismatch.)
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(
        root.join("package.json"),
        r#"{ "name": "infra", "dependencies": { "aws-cdk-lib": "^2.0.0" } }"#,
    )
    .unwrap();
    let adapter = mgc_cloud_adapter::adapter_for(root)
        .expect("adapter_for must not error")
        .expect("cdk package.json must detect a cloud adapter");
    let write_lock = ProjectWriteLock::acquire(root, std::time::Duration::from_secs(30)).unwrap();
    craft_identity_journal(
        root,
        "cloud",
        "terraform",
        ".terraform.lock.hcl",
        ".terraform.lock.hcl",
    );
    recover_interrupted_remove(&adapter, root, &write_lock)
        .await
        .expect_err("terraform journal via cdk command must fail closed");
    drop(write_lock);
}
