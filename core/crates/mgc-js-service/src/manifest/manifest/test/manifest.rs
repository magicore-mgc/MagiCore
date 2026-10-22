//! Workspace range round-trip contract tests.
//! Test contract round-trip workspace range.

use super::super::{parse_manifest, write_manifest};

/// Canonicalize the temp root because macOS mounts `/tmp` through `/var`.
/// Chuẩn hóa root tạm vì macOS mount `/tmp` qua `/var`.
fn real_root(dir: &tempfile::TempDir) -> std::path::PathBuf {
    dir.path()
        .canonicalize()
        .expect("canonicalize temp project root")
}

#[test]
fn project_manifest_without_version_round_trips_without_inventing_one() {
    let dir = tempfile::tempdir().expect("temporary Nuxt-style project");
    let root = real_root(&dir);
    let input = serde_json::json!({
        "name": "nuxt-app",
        "private": true,
        "type": "module",
        "scripts": {"dev": "nuxt dev"},
        "dependencies": {"nuxt": "^4.0.0"}
    });
    std::fs::write(root.join("package.json"), input.to_string()).expect("write manifest");

    let manifest = parse_manifest(&root).expect("parse unversioned application");
    assert!(manifest.version.is_none());
    assert_eq!(manifest.dependencies.len(), 1);
    write_manifest(&root, &manifest).expect("write unversioned application");

    let output: serde_json::Value = serde_json::from_slice(
        &std::fs::read(root.join("package.json")).expect("read updated manifest"),
    )
    .expect("updated JSON");
    assert!(output.get("version").is_none());
    assert_eq!(output["private"], true);
    assert_eq!(output["type"], input["type"]);
    assert_eq!(output["scripts"], input["scripts"]);
    assert_eq!(output["dependencies"], input["dependencies"]);
}

#[test]
fn project_manifest_still_rejects_invalid_explicit_version() {
    let dir = tempfile::tempdir().expect("temporary project");
    std::fs::write(
        dir.path().join("package.json"),
        r#"{"name":"app","version":"not-semver","private":true}"#,
    )
    .expect("write invalid version fixture");
    assert!(parse_manifest(dir.path()).is_err());
}

#[test]
fn manifest_writer_preserves_workspace_protocol_dependencies() {
    let dir = tempfile::tempdir().expect("temp project");
    let root = real_root(&dir);
    std::fs::write(
        root.join("package.json"),
        serde_json::json!({
            "name": "frontend",
            "version": "0.1.0",
            "dependencies": {
                "@core/shared": "workspace:*",
                "react": "^18.2.0"
            }
        })
        .to_string(),
    )
    .expect("write package.json");

    let manifest = parse_manifest(&root).expect("parse package.json");
    write_manifest(&root, &manifest).expect("rewrite package.json");

    let package: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join("package.json")).expect("read back"))
            .expect("valid JSON");
    assert_eq!(
        package["dependencies"]["@core/shared"], "workspace:*",
        "shared-service writes must preserve workspace protocol entries outside its registry graph"
    );
    assert_eq!(
        package["dependencies"]["react"], "^18.2.0",
        "shared-service writes must preserve registry ranges verbatim"
    );
}

#[cfg(unix)]
#[test]
fn parser_rejects_a_symlinked_package_json() {
    use std::os::unix::fs::symlink;

    let project = tempfile::tempdir().expect("temporary project");
    let outside = tempfile::tempdir().expect("external directory");
    let external_manifest = outside.path().join("package.json");
    std::fs::write(
        &external_manifest,
        r#"{"name":"outside","version":"1.0.0"}"#,
    )
    .expect("write external manifest");
    symlink(&external_manifest, project.path().join("package.json"))
        .expect("create manifest symlink");

    let error = parse_manifest(project.path()).expect_err("linked manifest must be refused");

    assert!(
        error
            .to_string()
            .contains("refusing to read unsafe package.json")
    );
}

#[test]
fn parser_rejects_a_manifest_larger_than_the_limit() {
    use std::io::Write;

    const MAX_MANIFEST_SIZE: u64 = crate::manifest::package_json::MAX_MANIFEST_SIZE;
    let project = tempfile::tempdir().expect("temporary project");
    let package_path = project.path().join("package.json");
    let file = std::fs::File::create(&package_path).expect("create package.json");
    file.set_len(MAX_MANIFEST_SIZE + 1)
        .expect("expand sparse package.json");
    drop(file);
    std::fs::OpenOptions::new()
        .write(true)
        .open(&package_path)
        .expect("open package.json")
        .write_all(b"{")
        .expect("write prefix");

    let error = parse_manifest(project.path()).expect_err("oversized manifest must be refused");

    assert!(error.to_string().contains("package.json is too large"));
}
