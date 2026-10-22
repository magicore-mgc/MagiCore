#![allow(clippy::unwrap_used)]

use crate::commands::core::shared::{lock_graph_matches_manifest_closure, lock_matches_manifest};
use mgc_lockfile::{Lockfile, Package};
use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

#[test]
fn test_lock_matches_manifest_when_versions_satisfy_ranges() {
    let mut manifest = Manifest::new("demo", Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("tailwindcss").unwrap(),
            VersionRange::parse("^4.3.0").unwrap(),
        ),
        false,
        false,
        false,
    );

    let mut lock = Lockfile::new();
    lock.packages.push(Package {
        name: "tailwindcss".into(),
        version: "4.3.2".into(),
        resolved: "https://registry.npmjs.org/tailwindcss/-/tailwindcss-4.3.2.tgz".into(),
        integrity: "sha256-test".into(),
        dependencies: vec![],
        ecosystem: mgc_lockfile::EcosystemTag::Web,
        registry: Some("npm://https://registry.npmjs.org".into()),
        ..Package::default()
    });

    assert!(lock_matches_manifest(&lock, &manifest));
}

#[test]
fn test_lock_matches_manifest_rejects_stale_version() {
    let mut manifest = Manifest::new("demo", Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("tailwindcss").unwrap(),
            VersionRange::parse("^5.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );

    let mut lock = Lockfile::new();
    lock.packages.push(Package {
        name: "tailwindcss".into(),
        version: "4.3.2".into(),
        resolved: "https://registry.npmjs.org/tailwindcss/-/tailwindcss-4.3.2.tgz".into(),
        integrity: "sha256-test".into(),
        dependencies: vec![],
        ecosystem: mgc_lockfile::EcosystemTag::Web,
        registry: Some("npm://https://registry.npmjs.org".into()),
        ..Package::default()
    });

    assert!(!lock_matches_manifest(&lock, &manifest));
}

#[test]
fn locked_graph_must_be_exactly_reachable_from_manifest_roots() {
    let mut manifest = Manifest::new("demo", Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("root-pkg").unwrap(),
            VersionRange::parse("^1.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let mut lock = Lockfile::new();
    lock.packages = vec![
        Package {
            name: "root-pkg".into(),
            version: "1.0.0".into(),
            dependencies: vec!["child-pkg@2.0.0".into()],
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            owner_core: Some("web".into()),
            ..Package::default()
        },
        Package {
            name: "child-pkg".into(),
            version: "2.0.0".into(),
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            owner_core: Some("web".into()),
            ..Package::default()
        },
        Package {
            name: "orphan-pkg".into(),
            version: "9.9.9".into(),
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            owner_core: Some("web".into()),
            ..Package::default()
        },
    ];

    assert!(
        !lock_graph_matches_manifest_closure(&lock, &manifest, "web"),
        "orphan entries must not be trusted as part of the install graph"
    );
    lock.packages.pop();
    lock.root_dependencies_by_owner
        .insert("web".into(), vec!["root-pkg@1.0.0".into()]);
    assert!(lock_graph_matches_manifest_closure(&lock, &manifest, "web"));
}

#[test]
fn locked_graph_uses_owner_scoped_exact_roots_and_peer_edges() {
    let mut manifest = Manifest::new("demo", Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("root-pkg").unwrap(),
            VersionRange::parse("^1.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let owner = "web";
    let mut lock = Lockfile::new();
    lock.root_dependencies_by_owner
        .insert(owner.into(), vec!["root-pkg@1.0.0".into()]);
    lock.packages = vec![
        Package {
            name: "root-pkg".into(),
            version: "1.0.0".into(),
            dependencies: vec!["bridge@1.0.0".into()],
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            owner_core: Some(owner.into()),
            ..Package::default()
        },
        Package {
            name: "bridge".into(),
            version: "1.0.0".into(),
            dependencies: vec!["root-pkg@1.5.0".into()],
            peers: Some(vec!["peer-pkg@2.0.0".into()]),
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            owner_core: Some(owner.into()),
            ..Package::default()
        },
        Package {
            name: "root-pkg".into(),
            version: "1.5.0".into(),
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            owner_core: Some(owner.into()),
            ..Package::default()
        },
        Package {
            name: "peer-pkg".into(),
            version: "2.0.0".into(),
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            owner_core: Some(owner.into()),
            ..Package::default()
        },
    ];

    assert!(
        lock_graph_matches_manifest_closure(&lock, &manifest, owner),
        "exact owner root must coexist with another satisfying transitive version and peer edge"
    );
    lock.packages[1].peers = None;
    assert!(
        !lock_graph_matches_manifest_closure(&lock, &manifest, owner),
        "a peer package omitted from graph edges must not be silently trusted"
    );
}

#[test]
fn locked_graph_ignores_unrelated_core_entries_in_unified_lock() {
    let mut manifest = Manifest::new("demo", Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("web-root").unwrap(),
            VersionRange::parse("^1.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let mut lock = Lockfile::new();
    lock.root_dependencies_by_owner
        .insert("web".into(), vec!["web-root@1.0.0".into()]);
    lock.packages = vec![
        Package {
            name: "web-root".into(),
            version: "1.0.0".into(),
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            owner_core: Some("web".into()),
            ..Package::default()
        },
        Package {
            name: "app-root".into(),
            version: "2.0.0".into(),
            ecosystem: mgc_lockfile::EcosystemTag::Dart,
            owner_core: Some("app".into()),
            ..Package::default()
        },
    ];

    assert!(lock_graph_matches_manifest_closure(&lock, &manifest, "web"));
}

#[test]
fn locked_graph_scopes_roots_to_selected_ecosystem_for_one_owner() {
    let mut manifest = Manifest::new("demo", Ecosystem::App);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("shared-name").unwrap(),
            VersionRange::parse("^1.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let owner = "app";
    let mut lock = Lockfile::new();
    lock.root_dependencies_by_owner.insert(
        owner.into(),
        vec![
            mgc_lockfile::format_root_pin(mgc_lockfile::EcosystemTag::Dart, "shared-name@1.0.0"),
            mgc_lockfile::format_root_pin(mgc_lockfile::EcosystemTag::Swift, "swift-root@2.0.0"),
        ],
    );
    lock.packages = vec![
        Package {
            name: "shared-name".into(),
            version: "1.0.0".into(),
            ecosystem: mgc_lockfile::EcosystemTag::Dart,
            owner_core: Some(owner.into()),
            ..Package::default()
        },
        Package {
            name: "swift-root".into(),
            version: "2.0.0".into(),
            ecosystem: mgc_lockfile::EcosystemTag::Swift,
            owner_core: Some(owner.into()),
            ..Package::default()
        },
    ];

    assert!(lock_graph_matches_manifest_closure(&lock, &manifest, owner));
}

#[test]
fn ecosystem_qualified_root_disambiguates_identical_package_ids() {
    let mut manifest = Manifest::new("demo", Ecosystem::App);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("shared-name").unwrap(),
            VersionRange::parse("^1.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let owner = "app";
    let mut lock = Lockfile::new();
    lock.root_dependencies_by_owner.insert(
        owner.into(),
        vec![mgc_lockfile::format_root_pin(
            mgc_lockfile::EcosystemTag::Dart,
            "shared-name@1.0.0",
        )],
    );
    lock.packages = vec![
        Package {
            name: "shared-name".into(),
            version: "1.0.0".into(),
            ecosystem: mgc_lockfile::EcosystemTag::Dart,
            owner_core: Some(owner.into()),
            ..Package::default()
        },
        Package {
            name: "shared-name".into(),
            version: "1.0.0".into(),
            ecosystem: mgc_lockfile::EcosystemTag::Swift,
            owner_core: Some(owner.into()),
            ..Package::default()
        },
    ];

    assert!(lock_graph_matches_manifest_closure(&lock, &manifest, owner));
}
