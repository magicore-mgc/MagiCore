//! Product-level CLI coverage for the shared outdated command.
//! Test CLI sản phẩm cho lệnh outdated dùng chung.
#![cfg(feature = "web")]
#![allow(clippy::unwrap_used)]

use std::process::Command;

#[test]
fn outdated_cli_json_returns_empty_array_for_web_project_without_dependencies() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(
        temp.path().join("package.json"),
        r#"{"name":"outdated-empty-fixture","version":"1.0.0"}"#,
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_mgc"))
        .args(["outdated", "--json"])
        .current_dir(temp.path())
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "mgc outdated --json failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap().trim(), "[]");
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("No '.mgc.core' found")
    );
}
