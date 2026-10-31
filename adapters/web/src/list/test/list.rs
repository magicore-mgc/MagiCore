//! `run_list` must report every declared section, not only `dependencies`.
//! `run_list` phải báo mọi section đã khai, không chỉ `dependencies`.
//!
//! Reported defect: `mgc list` printed 8 lines and silently dropped `react`,
//! `react-dom` and `typescript` even though all three were declared in
//! `package.json` and materialized in `node_modules`. (Lỗi đã báo: `mgc list`
//! in 8 dòng và âm thầm bỏ `react`, `react-dom`, `typescript` dù cả ba có
//! trong `package.json` và đã materialize trong `node_modules`.)

use std::path::Path;

use super::super::run_list;

/// Materialize a package the way install does: `node_modules/<name>` with a
/// `package.json` that proves the installed version.
/// (Materialize package như install: `node_modules/<name>` có `package.json`
/// chứng minh version đã cài.)
fn materialize(project_root: &Path, name: &str, version: &str) {
    let package_dir = project_root.join("node_modules").join(name);
    std::fs::create_dir_all(&package_dir).expect("create materialized package dir");
    std::fs::write(
        package_dir.join("package.json"),
        format!("{{\"name\":\"{name}\",\"version\":\"{version}\"}}"),
    )
    .expect("write materialized package.json");
}

fn write_manifest(project_root: &Path, manifest: &str) {
    std::fs::write(project_root.join("package.json"), manifest)
        .expect("write project package.json");
}

/// Every section is listed, with its installed version and section label.
/// (Mọi section đều được liệt kê, kèm version đã cài và nhãn section.)
#[tokio::test]
async fn lists_dependencies_from_every_manifest_section() {
    let dir = tempfile::tempdir().expect("create temp project");
    let root = dir.path();
    write_manifest(
        root,
        r#"{
            "name": "sectioned-app",
            "version": "1.0.0",
            "dependencies": {
                "react": "^18.2.0",
                "react-dom": "^18.2.0"
            },
            "devDependencies": {
                "typescript": "^5.4.0",
                "vite": "^5.2.0"
            },
            "optionalDependencies": {
                "fsevents": "^2.3.3"
            },
            "peerDependencies": {
                "@types/react": "^18.2.0"
            }
        }"#,
    );
    materialize(root, "react", "18.2.0");
    materialize(root, "react-dom", "18.2.0");
    materialize(root, "typescript", "5.4.5");
    materialize(root, "vite", "5.2.11");
    materialize(root, "fsevents", "2.3.3");
    materialize(root, "@types/react", "18.2.79");

    let listed = run_list(root).await.expect("list must succeed");
    let rows: Vec<(String, String, Option<String>, bool)> = listed
        .iter()
        .map(|package| {
            (
                package.id.name_str().to_string(),
                package.id.version().to_string(),
                package.section.clone(),
                package.is_dev,
            )
        })
        .collect();

    for expected in [
        ("react", "18.2.0", "dependencies"),
        ("react-dom", "18.2.0", "dependencies"),
        ("typescript", "5.4.5", "devDependencies"),
        ("vite", "5.2.11", "devDependencies"),
        ("fsevents", "2.3.3", "optionalDependencies"),
        ("@types/react", "18.2.79", "peerDependencies"),
    ] {
        let found = rows
            .iter()
            .find(|row| row.0 == expected.0)
            .unwrap_or_else(|| {
                panic!(
                    "'{}' is declared in {} and installed, so list must report it; got {rows:?}",
                    expected.0, expected.2
                )
            });
        assert_eq!(found.1, expected.1, "installed version for {}", expected.0);
        assert_eq!(
            found.2.as_deref(),
            Some(expected.2),
            "section for {}",
            expected.0
        );
    }

    assert!(
        rows.iter()
            .find(|row| row.0 == "typescript")
            .is_some_and(|row| row.3),
        "devDependencies entries keep the dev marker"
    );
    assert!(
        !rows
            .iter()
            .find(|row| row.0 == "react")
            .is_some_and(|row| row.3),
        "runtime dependencies must not be marked dev"
    );
}

/// A declared-but-not-installed dependency is not reported as installed.
/// (Dependency đã khai nhưng chưa cài không được báo là đã cài.)
#[tokio::test]
async fn declared_but_missing_package_is_not_listed() {
    let dir = tempfile::tempdir().expect("create temp project");
    let root = dir.path();
    write_manifest(
        root,
        r#"{
            "name": "partial-app",
            "version": "1.0.0",
            "dependencies": { "react": "^18.2.0", "react-dom": "^18.2.0" }
        }"#,
    );
    materialize(root, "react", "18.2.0");

    let listed = run_list(root).await.expect("list must succeed");
    let names: Vec<&str> = listed.iter().map(|p| p.id.name_str()).collect();
    assert_eq!(names, ["react"]);
}

/// Regression: an empty `node_modules/<name>` directory (interrupted
/// install, placeholder, or a scope directory) used to abort `mgc list`
/// with `MgError::Unsupported`, so EVERY other declared dependency
/// disappeared — the exact "list omits deps" symptom. The unverifiable
/// entry is skipped; the rest is still reported.
/// (Hồi quy: thư mục `node_modules/<name>` rỗng từng làm `mgc list` lỗi
/// `Unsupported`, khiến MỌI dependency khác biến mất — đúng triệu chứng
/// "list thiếu dep". Mục không xác minh được bị bỏ qua; phần còn lại vẫn
/// được báo.)
#[tokio::test]
async fn unverifiable_package_directory_does_not_hide_the_rest() {
    let dir = tempfile::tempdir().expect("create temp project");
    let root = dir.path();
    write_manifest(
        root,
        r#"{
            "name": "broken-app",
            "version": "1.0.0",
            "dependencies": {
                "react": "^18.2.0",
                "react-dom": "^18.2.0",
                "typescript": "^5.4.0"
            }
        }"#,
    );
    materialize(root, "react", "18.2.0");
    materialize(root, "react-dom", "18.2.0");
    // Empty directory with no package.json at all.
    // (Thư mục rỗng, không có package.json.)
    std::fs::create_dir_all(root.join("node_modules").join("typescript"))
        .expect("create empty package dir");

    let listed = run_list(root)
        .await
        .expect("an unverifiable directory must not fail the whole listing");
    let mut names: Vec<&str> = listed.iter().map(|p| p.id.name_str()).collect();
    names.sort_unstable();
    assert_eq!(
        names,
        ["react", "react-dom"],
        "verified packages stay listed, the placeholder does not"
    );
}
