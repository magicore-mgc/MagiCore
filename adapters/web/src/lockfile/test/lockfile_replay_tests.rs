// lockfile_replay_tests — exact graph replay and embedded core ownership.
// lockfile_replay_tests — kiểm thử exact graph replay and embedded core ownership.
use super::*;

#[test]
fn web_lockfile_replay_restores_transitive_exact_version_and_peer_edges() {
    let root = PackageId::parse("app@1.0.0").unwrap();
    let scoped = PackageId::parse("@scope/lib@2.0.0").unwrap();
    let nested_v1 = PackageId::parse("nested@1.0.0").unwrap();
    let nested_v2 = PackageId::parse("nested@2.0.0").unwrap();
    let peer = PackageId::parse("peer@1.0.0").unwrap();
    let mut lock = Lockfile::new();
    lock.packages = vec![
        mgc_lockfile::Package {
            owner_core: Some("web".into()),
            name: root.name_str().into(),
            version: root.version().to_string(),
            dependencies: vec![scoped.to_string()],
            peers: Some(vec![peer.to_string()]),
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            registry: Some("https://registry.npmjs.org".into()),
            ..Default::default()
        },
        mgc_lockfile::Package {
            owner_core: Some("web".into()),
            name: scoped.name_str().into(),
            version: scoped.version().to_string(),
            dependencies: vec![nested_v2.to_string()],
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            registry: Some("https://registry.npmjs.org".into()),
            ..Default::default()
        },
        mgc_lockfile::Package {
            owner_core: Some("web".into()),
            name: nested_v1.name_str().into(),
            version: nested_v1.version().to_string(),
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            registry: Some("https://registry.npmjs.org".into()),
            ..Default::default()
        },
        mgc_lockfile::Package {
            owner_core: Some("web".into()),
            name: nested_v2.name_str().into(),
            version: nested_v2.version().to_string(),
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            registry: Some("https://registry.npmjs.org".into()),
            ..Default::default()
        },
        mgc_lockfile::Package {
            owner_core: Some("web".into()),
            name: peer.name_str().into(),
            version: peer.version().to_string(),
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            registry: Some("https://registry.npmjs.org".into()),
            ..Default::default()
        },
    ];
    mgc_lockfile::update_owner_root_pins(
        &mut lock,
        "web",
        mgc_lockfile::EcosystemTag::Web,
        [root.to_string()],
    );

    let mut manifest = Manifest::new("demo", mgc_types::ecosystem::Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("app").unwrap(),
            mgc_types::VersionRange::parse("^1.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let graph = build_graph_from_lockfile(&lock, &manifest, "web")
        .unwrap()
        .expect("the complete lock closure should replay");
    let packages: std::collections::HashMap<_, _> = graph
        .packages
        .iter()
        .map(|package| (package.id.to_string(), package))
        .collect();

    assert_eq!(
        packages.len(),
        4,
        "replay the root and all reachable entries"
    );
    assert!(!packages.contains_key(&nested_v1.to_string()));
    assert_eq!(packages[&root.to_string()].deps, vec![scoped.clone()]);
    assert_eq!(packages[&root.to_string()].peer_deps, vec![peer.clone()]);
    assert_eq!(packages[&scoped.to_string()].deps, vec![nested_v2.clone()]);
    assert!(packages[&root.to_string()].direct);
    assert_eq!(
        packages.values().filter(|package| package.direct).count(),
        1
    );

    let mut incomplete_lock = lock.clone();
    incomplete_lock
        .packages
        .retain(|package| !(package.name == "nested" && package.version == "2.0.0"));
    assert!(
        build_graph_from_lockfile(&incomplete_lock, &manifest, "web")
            .unwrap()
            .is_none(),
        "a missing exact-version edge must miss the lock path instead of disappearing"
    );
}

#[test]
fn embedded_web_engine_preserves_lock_entries_owned_by_another_core() {
    let dir = tempdir_real().unwrap();
    mgc_config::project::ProjectConfig::write_core_marker_at(dir.path(), "clo").unwrap();
    let path = dir.path().join("mgc.lock");
    let mut lock = Lockfile::new();
    lock.packages.push(mgc_lockfile::Package {
        owner_core: Some("web".to_string()),
        name: "react".into(),
        version: "18.2.0".into(),
        ecosystem: mgc_lockfile::EcosystemTag::Web,
        registry: Some("https://registry.npmjs.org".into()),
        ..Default::default()
    });
    std::fs::write(
        &path,
        mgc_lockfile::writer::serialize_lockfile(&lock).unwrap(),
    )
    .unwrap();
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: PackageId::new(
                PackageName::new("constructs").unwrap(),
                Version::parse("10.0.0").unwrap(),
            ),
            integrity: "sha512-cloud".into(),
            tarball_url: String::new(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };

    write_web_lockfile_with_state(dir.path(), &graph, "https://registry.npmjs.org").unwrap();
    let written = mgc_lockfile::parser::load_lockfile(&path).unwrap();
    assert!(written.packages.iter().any(|package| {
        package.owner_core.as_deref() == Some("web") && package.name == "react"
    }));
    assert!(written.packages.iter().any(|package| {
        package.owner_core.as_deref() == Some("clo") && package.name == "constructs"
    }));
}
