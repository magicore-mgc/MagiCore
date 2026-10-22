// lockfile_v3_write_tests — v3 writer and owner roots.
// lockfile_v3_write_tests — kiểm thử v3 writer and owner roots.
use super::*;

#[test]
fn test_write_web_lockfile_with_state_skips_rewrite_when_unchanged() {
    let dir = tempdir_real().unwrap();
    let package_id = PackageId::new(
        PackageName::new("react").unwrap(),
        Version::parse("18.2.0").unwrap(),
    );
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: package_id,
            integrity: "sha512-demo".into(),
            tarball_url: String::new(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };

    write_web_lockfile_with_state(dir.path(), &graph, "https://registry.npmjs.org").unwrap();
    let lock_path = dir.path().join("mgc.lock");
    let first_lock_modified = std::fs::metadata(&lock_path).unwrap().modified().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    write_web_lockfile_with_state(dir.path(), &graph, "https://registry.npmjs.org").unwrap();
    let second_lock_modified = std::fs::metadata(&lock_path).unwrap().modified().unwrap();

    assert_eq!(first_lock_modified, second_lock_modified);
}

#[test]
fn web_writer_persists_owner_scoped_roots_and_versioned_peer_edges() {
    let dir = tempdir_real().unwrap();
    let package_id = |name: &str, version: &str| {
        PackageId::new(
            PackageName::new(name).unwrap(),
            Version::parse(version).unwrap(),
        )
    };
    let button = package_id("button", "1.0.0");
    let react = package_id("react", "18.2.0");
    let graph = ResolvedGraph {
        packages: vec![
            ResolvedPackage {
                id: button,
                integrity: "sha512-button".into(),
                tarball_url: "https://registry.example/button.tgz".into(),
                deps: vec![],
                peer_deps: vec![react.clone()],
                direct: true,
                dev: false,
            },
            ResolvedPackage {
                id: react,
                integrity: "sha512-react".into(),
                tarball_url: "https://registry.example/react.tgz".into(),
                deps: vec![],
                peer_deps: vec![],
                direct: false,
                dev: false,
            },
        ],
    };

    write_web_lockfile_with_state(dir.path(), &graph, "https://registry.example").unwrap();
    let lock = mgc_lockfile::parser::load_lockfile(&dir.path().join("mgc.lock")).unwrap();
    assert_eq!(
        lock.root_dependencies_by_owner["web"],
        vec![mgc_lockfile::format_root_pin(
            mgc_lockfile::EcosystemTag::Web,
            "button@1.0.0",
        )]
    );
    let button = lock
        .packages
        .iter()
        .find(|package| package.name == "button")
        .unwrap();
    assert_eq!(
        button.peers.as_deref(),
        Some(["react@18.2.0".to_string()].as_slice())
    );
}

