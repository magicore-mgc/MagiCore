//! V4 lock schema projection and loss-prevention tests.
//! Test projection schema lock V4 và ngăn mất dữ liệu.

use super::support::*;

#[test]
fn v4_schema_writes_v4_lock() {
    let dir = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::write_core_marker_at(dir.path(), "lib").unwrap();
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"lib\"\necosystem = \"lib\"\n[lock]\nschema = \"v4\"\n",
    )
    .unwrap();
    write_canonical_lock(
        dir.path(),
        EcosystemTag::Python,
        vec![v4_lock_package("requests", "2.0.0")],
    )
    .unwrap();
    let text = std::fs::read_to_string(dir.path().join("mgc.lock")).unwrap();
    assert_eq!(
        mgc_lockfile::migrate::detect_lockfile_version(&text).unwrap(),
        4
    );
    let doc = mgc_lockfile::canonical::parse_v4_document(&text).unwrap();
    assert!(
        mgc_lockfile::policy::verify_v4_math(&doc)
            .unwrap()
            .digest_ok
    );
    assert!(doc.packages.iter().any(|p| p.key.name == "requests"));
}

#[test]
fn v4_new_owner_projection_is_empty_until_its_own_packages_are_written() {
    let dir = tempfile::tempdir().unwrap();
    let config = dir.path().join("mgc.toml");
    std::fs::write(
        &config,
        "name = \"library\"\necosystem = \"lib\"\n[lock]\nschema = \"v4\"\n",
    )
    .unwrap();
    write_canonical_lock_with_roots(
        dir.path(),
        EcosystemTag::Python,
        vec![v4_lock_package("requests", "2.0.0")],
        vec!["requests@2.0.0".into()],
    )
    .unwrap();
    std::fs::write(&config, "name = \"agent\"\necosystem = \"ai\"\n[ai]\nframework = \"python-agent\"\n[lock]\nschema = \"v4\"\n").unwrap();
    assert!(
        read_existing_lock(dir.path()).unwrap().packages.is_empty(),
        "a new AI owner cannot inherit the Lib dependency rows"
    );
    write_canonical_lock_with_roots(
        dir.path(),
        EcosystemTag::Python,
        vec![v4_lock_package("six", "1.17.0")],
        vec!["six@1.17.0".into()],
    )
    .unwrap();
    let owned = read_existing_lock(dir.path()).unwrap();
    assert_eq!(owned.packages.len(), 1);
    assert_eq!(owned.packages[0].name, "six");
    let doc = mgc_lockfile::canonical::parse_v4_document(
        &std::fs::read_to_string(dir.path().join("mgc.lock")).unwrap(),
    )
    .unwrap();
    assert_eq!(doc.packages.len(), 2);
    assert!(doc.root_dependencies_by_owner.contains_key("lib"));
    assert!(doc.root_dependencies_by_owner.contains_key("ai"));
}

#[test]
fn v4_schema_reads_existing_lib_package_rows_in_document_order() {
    let dir = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::write_core_marker_at(dir.path(), "lib").unwrap();
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"lib\"\necosystem = \"lib\"\n[lock]\nschema = \"v4\"\n",
    )
    .unwrap();
    write_canonical_lock_with_roots(
        dir.path(),
        EcosystemTag::Python,
        vec![
            v4_lock_package("requests", "2.0.0"),
            v4_lock_package("urllib3", "2.0.0"),
        ],
        vec!["requests@2.0.0".to_string(), "urllib3@2.0.0".to_string()],
    )
    .unwrap();

    let text = std::fs::read_to_string(dir.path().join("mgc.lock")).unwrap();
    let document = mgc_lockfile::canonical::parse_v4_document(&text).unwrap();
    let expected_order = document
        .packages
        .iter()
        .map(|package| package.key.name.clone())
        .collect::<Vec<_>>();
    let lock = read_existing_lock(dir.path()).unwrap();
    let projected_order = lock
        .packages
        .iter()
        .map(|package| package.name.clone())
        .collect::<Vec<_>>();
    assert_eq!(projected_order, expected_order);
    assert_eq!(
        read_existing_lock(dir.path()).unwrap().packages,
        lock.packages
    );

    let requests = lock
        .packages
        .iter()
        .find(|package| package.name == "requests")
        .expect("V4 root package should project into the lib adapter model");
    assert_eq!(requests.version, "2.0.0");
    assert_eq!(requests.ecosystem, EcosystemTag::Python);
    assert!(requests.integrity.starts_with("sha512-"));
    assert_eq!(requests.owner_core.as_deref(), Some("lib"));
}

#[test]
fn v4_schema_refuses_lossy_lane_rows() {
    let dir = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::write_core_marker_at(dir.path(), "lib").unwrap();
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"lib\"\necosystem = \"lib\"\n[lock]\nschema = \"v4\"\n",
    )
    .unwrap();
    let mut bare = v4_lock_package("bare", "1.0.0");
    bare.registry = None;
    bare.integrity = String::new();
    let err = write_canonical_lock(dir.path(), EcosystemTag::Python, vec![bare]).unwrap_err();
    assert!(
        err.to_string().contains("lossy"),
        "expected lossy refusal, got: {err}"
    );
    assert!(
        !dir.path().join("mgc.lock").exists(),
        "refused v4 write must not publish any lock"
    );
}

// Legacy and SRI encodings must reconstruct the same verified digest.
// Lock V4 đổi encoding không được làm mất hash hoặc phá native frozen install.
