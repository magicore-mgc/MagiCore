#![allow(clippy::unwrap_used)]

use std::process::Command;
use tempfile::TempDir;

fn run_mgc(binary: &str, args: &[&str], cwd: &std::path::Path, home: &std::path::Path) -> String {
    let output = Command::new(binary)
        .args(args)
        .current_dir(cwd)
        .env("HOME", home)
        .env("USERPROFILE", home)
        .env("MGC_DATA_DIR", home.join("data"))
        .env("MGC_CACHE_DIR", home.join("cache"))
        .env("MGC_TEMPLATES_DIR", home.join("templates"))
        .env("CARGO_NET_OFFLINE", "true")
        .output()
        .expect("spawn mgc");
    let output_text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.status.success(),
        "mgc {args:?} failed:\n{output_text}"
    );
    output_text
}

#[test]
fn recursive_build_recreates_outputs_removed_after_successful_build() {
    let root = TempDir::new().unwrap();
    let home = TempDir::new().unwrap();
    let package = root.path().join("packages/audit-one");
    std::fs::create_dir_all(package.join("src")).unwrap();
    std::fs::write(
        root.path().join("mgc.toml"),
        "name = \"audit-mono\"\necosystem = \"lib\"\n[lib]\nlanguage = \"rust\"\n",
    )
    .unwrap();
    std::fs::write(
        root.path().join("magicore.workspace.toml"),
        "mode = \"monorepo\"\n[layout]\napps_dir = \"apps\"\npackages_dir = \"packages\"\n",
    )
    .unwrap();
    std::fs::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = []\n",
    )
    .unwrap();
    std::fs::write(
        package.join("mgc.toml"),
        "name = \"audit-one\"\necosystem = \"lib\"\n[lib]\nlanguage = \"rust\"\n",
    )
    .unwrap();
    std::fs::write(
        package.join("Cargo.toml"),
        "[package]\nname = \"audit_one\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[workspace]\n",
    )
    .unwrap();
    std::fs::write(
        package.join("src/lib.rs"),
        "pub fn answer() -> i32 { 42 }\n",
    )
    .unwrap();

    let binary = env!("CARGO_BIN_EXE_mgc");
    run_mgc(binary, &["install", "--core", "lib"], &package, home.path());
    run_mgc(binary, &["--recursive", "build"], root.path(), home.path());

    let output = package.join("target/debug/deps");
    let artifacts: Vec<_> = std::fs::read_dir(&output)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("libaudit_one")
        })
        .collect();
    assert!(
        !artifacts.is_empty(),
        "first build must produce Rust library outputs"
    );
    for artifact in artifacts {
        std::fs::remove_file(artifact).unwrap();
    }

    run_mgc(binary, &["--recursive", "build"], root.path(), home.path());

    assert!(
        std::fs::read_dir(&output)
            .unwrap()
            .filter_map(Result::ok)
            .any(|entry| entry
                .path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("libaudit_one")),
        "a successful cached build must not leave required compiler outputs missing"
    );
}
