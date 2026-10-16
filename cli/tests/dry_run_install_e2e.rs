#![allow(clippy::unwrap_used)]

use std::process::Command;
use tempfile::TempDir;

#[test]
fn web_install_dry_run_fails_closed_without_project_or_cache_writes() {
    let project = TempDir::new().unwrap();
    let home = TempDir::new().unwrap();
    let data = home.path().join("data");
    let manifest = "{\"name\":\"dry-run-probe\",\"version\":\"1.0.0\",\"dependencies\":{}}\n";
    std::fs::write(project.path().join("package.json"), manifest).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_mgc"))
        .args(["install", "--dry-run", "--core", "web"])
        .current_dir(project.path())
        .env("HOME", home.path())
        .env("USERPROFILE", home.path())
        .env("MGC_DATA_DIR", &data)
        .output()
        .expect("spawn mgc");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        !output.status.success(),
        "unsupported dry-run must fail closed:\n{text}"
    );
    assert!(text.contains("dry-run is not supported") && text.contains("no changes were made"));
    assert_eq!(
        std::fs::read_to_string(project.path().join("package.json")).unwrap(),
        manifest
    );
    assert!(!project.path().join("mgc.lock").exists());
    assert!(!project.path().join("node_modules").exists());
    assert!(!project.path().join(".magicore").exists());
    assert!(!data.exists());
}
