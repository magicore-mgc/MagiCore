#![allow(clippy::unwrap_used)]

use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

fn cargo_target_dir(project: &Path, configured: Option<&Path>) -> PathBuf {
    match configured {
        Some(path) if path.is_absolute() => path.to_path_buf(),
        Some(path) => project.join(path),
        None => project.join("target"),
    }
}

#[test]
fn cargo_target_dir_resolves_absolute_relative_and_default_paths() {
    let fixture = TempDir::new().unwrap();
    let project = fixture.path().join("package");
    let shared = fixture.path().join("shared-target");

    assert_eq!(
        cargo_target_dir(&project, Some(&shared)),
        shared,
        "absolute Cargo target directories stay absolute"
    );
    assert_eq!(
        cargo_target_dir(&project, Some(Path::new("target-cache"))),
        project.join("target-cache"),
        "relative Cargo target directories resolve from the package workspace"
    );
    assert_eq!(
        cargo_target_dir(&project, None),
        project.join("target"),
        "without an override, Cargo artifacts remain project-local"
    );
}

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

    let configured_target = std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from);
    let output = cargo_target_dir(&package, configured_target.as_deref()).join("debug/deps");
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
