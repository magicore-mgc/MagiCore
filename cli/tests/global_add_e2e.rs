#![allow(clippy::unwrap_used)]

use std::process::Command;
use tempfile::TempDir;

#[test]
fn global_add_fails_closed_before_changing_project_or_user_data() {
    let project = TempDir::new().unwrap();
    let home = TempDir::new().unwrap();
    let data = home.path().join("data");
    let package_json =
        "{\"name\":\"global-add-probe\",\"version\":\"1.0.0\",\"dependencies\":{}}\n";
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"global-add-probe\"\necosystem = \"web\"\n\n[web]\nframework = \"node\"\n",
    )
    .unwrap();
    std::fs::write(project.path().join("package.json"), package_json).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_mgc"))
        .args(["add", "opencode", "--global"])
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
        "unsupported global add must fail:\n{text}"
    );
    assert!(text.contains("mgc add --global` is not supported"));
    assert!(text.contains("mgc tool install npm:<package>"));
    assert!(text.contains("no project changes were made"));
    assert_eq!(
        std::fs::read_to_string(project.path().join("package.json")).unwrap(),
        package_json
    );
    assert!(!project.path().join("mgc.lock").exists());
    assert!(!project.path().join("node_modules").exists());
    assert!(!project.path().join(".magicore").exists());
    assert!(!data.exists());
}
