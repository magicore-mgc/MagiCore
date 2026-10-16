#![allow(clippy::unwrap_used)]

use std::process::Command;
use tempfile::TempDir;

#[test]
fn bench_no_lock_refuses_without_mutating_lockfile() {
    let project = TempDir::new().unwrap();
    let home = TempDir::new().unwrap();
    let data = home.path().join("data");
    let lock = "version = 4\n[project]\nname = \"bench-safety\"\n";
    std::fs::write(project.path().join("mgc.lock"), lock).unwrap();
    std::fs::write(
        project.path().join("package.json"),
        "{\"name\":\"bench-safety\",\"version\":\"1.0.0\",\"dependencies\":{}}\n",
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_mgc"))
        .args(["bench", "--no-lock"])
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
        "unsafe mode must fail closed: {text}"
    );
    assert!(text.contains("bench --no-lock is disabled"), "{text}");
    assert_eq!(
        std::fs::read_to_string(project.path().join("mgc.lock")).unwrap(),
        lock
    );
    assert!(!data.exists());
}
