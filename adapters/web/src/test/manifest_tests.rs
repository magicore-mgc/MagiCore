// manifest_tests — split from unit_tests.rs by behavior/subject (P07).
// manifest_tests — tách từ unit_tests.rs theo behavior/chủ đề (P07).
use super::*;

#[test]
fn test_web_adapter() {
    assert_eq!(
        WebAdapter::new().unwrap().registry_url,
        "https://registry.npmjs.org"
    );
}

#[test]
fn test_package_json() {
    let p = PackageJson::new("t".into(), "1.0.0".into());
    assert_eq!(p.name, "t");
}
#[test]
fn test_can_handle() {
    let dir = tempdir_real().unwrap();
    PackageJson::new("t".into(), "1.0.0".into())
        .save(&dir.path().join("package.json"))
        .unwrap();
    assert!(WebAdapter::new().unwrap().can_handle(dir.path()));
}

#[tokio::test]
async fn test_parse_manifest_ignores_workspace_protocol_dependencies() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
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
    .unwrap();

    let adapter = WebAdapter::new().unwrap();
    let manifest = adapter.parse_manifest(dir.path()).await.unwrap();
    assert!(manifest.find_dep("react").is_some());
    assert!(manifest.find_dep("@core/shared").is_none());
}

#[test]
fn web_manifest_writer_preserves_workspace_protocol_dependencies() {
    let dir = tempdir_real().unwrap();
    let package_path = dir.path().join("package.json");
    std::fs::write(
        &package_path,
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
    .unwrap();
    let manifest = parse_manifest(dir.path()).unwrap();

    write_manifest(dir.path(), &manifest).unwrap();

    let package: serde_json::Value =
        serde_json::from_slice(&std::fs::read(package_path).unwrap()).unwrap();
    assert_eq!(
        package["dependencies"]["@core/shared"], "workspace:*",
        "MGC writes must preserve workspace protocol entries outside its registry graph"
    );
    assert_eq!(package["dependencies"]["react"], "^18.2.0");
}

#[test]
fn web_manifest_writer_refuses_registry_replacement_of_workspace_dependency() {
    let dir = tempdir_real().unwrap();
    let package_path = dir.path().join("package.json");
    let original = serde_json::json!({
        "name": "frontend",
        "version": "0.1.0",
        "dependencies": { "@core/shared": "workspace:*" }
    })
    .to_string();
    std::fs::write(&package_path, &original).unwrap();
    let mut manifest = Manifest::new("frontend", mgc_types::Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("@core/shared").unwrap(),
            VersionRange::parse("^1.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );

    assert!(write_manifest(dir.path(), &manifest).is_err());
    assert_eq!(
        std::fs::read_to_string(package_path).unwrap(),
        original,
        "source conversion needs an explicit operation and cannot be silently accepted"
    );
}

#[test]
fn web_manifest_writer_does_not_replace_malformed_existing_json() {
    let dir = tempdir_real().unwrap();
    let package_path = dir.path().join("package.json");
    let malformed = b"{ this is not package json";
    std::fs::write(&package_path, malformed).unwrap();
    let manifest = Manifest::new("frontend", mgc_types::Ecosystem::Web);

    let error = write_manifest(dir.path(), &manifest)
        .expect_err("invalid existing package metadata must not fall back to an empty manifest");

    assert!(error.to_string().contains("read existing package.json"));
    assert_eq!(std::fs::read(package_path).unwrap(), malformed);
}

#[test]
fn web_manifest_writer_can_atomically_create_a_missing_package_json() {
    let dir = tempdir_real().unwrap();
    let manifest = Manifest::new("new-web-project", mgc_types::Ecosystem::Web);

    write_manifest(dir.path(), &manifest).unwrap();

    let parsed = parse_manifest(dir.path()).unwrap();
    assert_eq!(parsed.name, "new-web-project");
    assert!(dir.path().join("package.json").is_file());
}

#[cfg(unix)]
#[test]
fn web_manifest_writer_refuses_symlinked_package_json() {
    use std::os::unix::fs::symlink;

    let dir = tempdir_real().unwrap();
    let outside = tempdir_real().unwrap();
    let target = outside.path().join("package.json");
    std::fs::write(&target, "{\"sentinel\":true}").unwrap();
    symlink(&target, dir.path().join("package.json")).unwrap();
    let manifest = Manifest::new("frontend", mgc_types::Ecosystem::Web);

    assert!(write_manifest(dir.path(), &manifest).is_err());
    assert_eq!(
        std::fs::read_to_string(target).unwrap(),
        "{\"sentinel\":true}"
    );
}

#[tokio::test]
async fn test_list_uses_materialized_version_not_lock_pin_as_installed_state() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
            "version": "0.1.0",
            "dependencies": {
                "tailwindcss": "^4.3.0"
            }
        })
        .to_string(),
    )
    .unwrap();

    let package_dir = dir.path().join("node_modules").join("tailwindcss");
    std::fs::create_dir_all(&package_dir).unwrap();
    std::fs::write(
        package_dir.join("package.json"),
        "{\"name\":\"tailwindcss\",\"version\":\"4.3.3\"}",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("mgc.lock"),
        serde_json::json!({
            "version": "2",
            "metadata": {
                "generated_at": "2024-01-01T00:00:00Z",
                "generator": "mgc/1.0.0",
                "lockfile_hash": ""
            },
            "package": [{
                "name": "tailwindcss",
                "version": "4.3.2",
                "resolved": "https://registry.npmjs.org/tailwindcss/-/tailwindcss-4.3.2.tgz",
                "integrity": "sha256-test",
                "dependencies": []
            }]
        })
        .to_string(),
    )
    .unwrap();

    let adapter = WebAdapter::new().unwrap();
    let installed = adapter.list(dir.path()).await.unwrap();
    assert_eq!(installed.len(), 1);
    assert_eq!(installed[0].id.name_str(), "tailwindcss");
    assert_eq!(installed[0].id.version().to_string(), "4.3.3");
    assert_eq!(installed[0].integrity, None);
}

#[tokio::test]
async fn test_list_fails_closed_when_installed_version_is_unknown() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({"name":"demo","version":"0.1.0","dependencies":{"mystery":"^1.0.0"}})
            .to_string(),
    )
    .unwrap();
    let package_dir = dir.path().join("node_modules/mystery");
    std::fs::create_dir_all(&package_dir).unwrap();
    std::fs::write(package_dir.join("package.json"), r#"{"name":"mystery"}"#).unwrap();

    let error = WebAdapter::new()
        .unwrap()
        .list(dir.path())
        .await
        .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("cannot verify the installed version")
    );
}

#[tokio::test]
async fn test_list_does_not_guess_installed_version_from_lockfile() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({"name":"demo","version":"0.1.0","dependencies":{"mystery":"^1.0.0"}})
            .to_string(),
    )
    .unwrap();
    let package_dir = dir.path().join("node_modules/mystery");
    std::fs::create_dir_all(&package_dir).unwrap();
    std::fs::write(package_dir.join("package.json"), r#"{"name":"mystery"}"#).unwrap();
    std::fs::write(
        dir.path().join("mgc.lock"),
        serde_json::json!({
            "version": "2",
            "metadata": {"generated_at":"2024-01-01T00:00:00Z","generator":"mgc/1.0.0","lockfile_hash":""},
            "package": [{"name":"mystery","version":"1.2.3","resolved":"https://registry.npmjs.org/mystery/-/mystery-1.2.3.tgz","integrity":"sha256-test","dependencies":[]}]
        }).to_string(),
    ).unwrap();

    let error = WebAdapter::new()
        .unwrap()
        .list(dir.path())
        .await
        .unwrap_err();
    assert!(error.to_string().contains("a lock pin alone is not proof"));
}

#[test]
fn test_installed_package_version_reads_real_version() {
    let dir = tempdir_real().unwrap();
    let pkg_dir = dir.path().join("node_modules").join("zod");
    std::fs::create_dir_all(&pkg_dir).unwrap();
    std::fs::write(
        pkg_dir.join("package.json"),
        br#"{"name":"zod","version":"4.4.3"}"#,
    )
    .unwrap();

    assert_eq!(
        installed_package_version(&pkg_dir).unwrap().to_string(),
        "4.4.3"
    );
}

#[test]
fn test_installed_package_matches_version() {
    let dir = tempdir_real().unwrap();
    let pkg_dir = dir.path().join("node_modules").join("zod");
    std::fs::create_dir_all(&pkg_dir).unwrap();
    std::fs::write(
        pkg_dir.join("package.json"),
        br#"{"name":"zod","version":"4.4.3"}"#,
    )
    .unwrap();
    let package_id = PackageId::new(
        PackageName::new("zod").unwrap(),
        Version::parse("4.4.3").unwrap(),
    );
    assert!(installed_package_matches(&pkg_dir, &package_id));
}
