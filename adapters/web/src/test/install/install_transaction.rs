//! Web install transaction integration tests.
//! Test tích hợp transaction cài package Web.

use super::*;

#[tokio::test]
async fn test_install_finalizes_lock_and_cleans_staging_tmp() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
            "version": "0.1.0",
            "dependencies": {
                "react": "^18.2.0",
                "@types/react": "^19.2.17"
            }
        })
        .to_string(),
    )
    .unwrap();

    let react = PackageId::new(
        PackageName::new("react").unwrap(),
        Version::parse("18.2.0").unwrap(),
    );
    let react_types = PackageId::new(
        PackageName::new("@types/react").unwrap(),
        Version::parse("19.2.17").unwrap(),
    );

    let react_integrity = seed_cached_tarball_with_files(
        dir.path(),
        &react,
        &[
            (
                "package/package.json",
                br#"{"name":"react","version":"18.2.0"}"#.as_slice(),
            ),
            ("package/index.js", b"export default 'react';"),
        ],
    );
    let react_types_integrity = seed_cached_tarball_with_files(
        dir.path(),
        &react_types,
        &[
            (
                "package/package.json",
                br#"{"name":"@types/react","version":"19.2.17"}"#.as_slice(),
            ),
            ("package/index.d.ts", b"export = React;"),
        ],
    );

    let graph = ResolvedGraph {
        packages: vec![
            ResolvedPackage {
                id: react.clone(),
                integrity: react_integrity,
                tarball_url: String::new(),
                deps: vec![],
                peer_deps: vec![],
                direct: true,
                dev: false,
            },
            ResolvedPackage {
                id: react_types.clone(),
                integrity: react_types_integrity,
                tarball_url: String::new(),
                deps: vec![],
                peer_deps: vec![],
                direct: true,
                dev: true,
            },
        ],
    };

    let adapter = WebAdapter::new().unwrap();
    let summary = adapter
        .install(&graph, dir.path(), InstallOptions::default())
        .await
        .unwrap();
    assert_eq!(summary.added.len(), 2);
    assert!(dir.path().join("node_modules/react/index.js").exists());
    assert!(
        dir.path()
            .join("node_modules/@types/react/index.d.ts")
            .exists()
    );

    let lock = std::fs::read_to_string(dir.path().join("mgc.lock")).unwrap();
    let parsed: Lockfile = mgc_lockfile::parse_lockfile(&lock).unwrap();
    // Deliberate v3 bump (Phase 1): the web writer now emits schema v3.
    // Nâng lên v3 có chủ đích (Phase 1): web writer giờ ghi schema v3.
    assert_eq!(parsed.version, "3");
    assert_eq!(parsed.packages.len(), 2);

    let tmp_dir = dir
        .path()
        .join(".magicore")
        .join("cache")
        .join("web")
        .join("tmp");
    let lingering_entries = std::fs::read_dir(&tmp_dir)
        .unwrap()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    assert!(
        lingering_entries.is_empty(),
        "expected staging tmp to be cleaned, found {:?}",
        lingering_entries
            .iter()
            .map(|entry| entry.path())
            .collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn test_install_failure_does_not_materialize_partial_node_modules() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
            "version": "0.1.0",
            "dependencies": {
                "react": "^18.2.0",
                "tailwindcss": "^4.3.0"
            }
        })
        .to_string(),
    )
    .unwrap();

    let react = PackageId::new(
        PackageName::new("react").unwrap(),
        Version::parse("18.2.0").unwrap(),
    );
    let missing = PackageId::new(
        PackageName::new("tailwindcss").unwrap(),
        Version::parse("4.3.2").unwrap(),
    );

    let react_integrity = seed_cached_tarball_with_files(
        dir.path(),
        &react,
        &[
            (
                "package/package.json",
                br#"{"name":"react","version":"18.2.0"}"#.as_slice(),
            ),
            ("package/index.js", b"export default 'react';"),
        ],
    );

    let graph = ResolvedGraph {
        packages: vec![
            ResolvedPackage {
                id: react,
                integrity: react_integrity,
                tarball_url: String::new(),
                deps: vec![],
                peer_deps: vec![],
                direct: true,
                dev: false,
            },
            ResolvedPackage {
                id: missing,
                integrity: String::new(),
                tarball_url: "http://127.0.0.1:9/unreachable.tgz".into(),
                deps: vec![],
                peer_deps: vec![],
                direct: true,
                dev: false,
            },
        ],
    };

    let adapter = WebAdapter::build("http://127.0.0.1:9".into(), None, vec![], None);
    let _err = adapter
        .install(&graph, dir.path(), InstallOptions::default())
        .await
        .unwrap_err();
    assert!(!dir.path().join("node_modules/react").exists());

    let lock = std::fs::read_to_string(dir.path().join("mgc.lock")).unwrap();
    let parsed: Lockfile = mgc_lockfile::parse_lockfile(&lock).unwrap();
    // Deliberate v3 bump (Phase 1): the web writer now emits schema v3.
    // Nâng lên v3 có chủ đích (Phase 1): web writer giờ ghi schema v3.
    assert_eq!(parsed.version, "3");
    assert_eq!(parsed.packages.len(), 2);
}
