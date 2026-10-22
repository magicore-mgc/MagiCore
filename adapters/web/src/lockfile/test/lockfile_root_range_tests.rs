// lockfile_root_range_tests — manifest root range classification.
// lockfile_root_range_tests — kiểm thử manifest root range classification.
use super::*;

#[test]
fn web_direct_root_classification_checks_the_manifest_range() {
    let mut manifest = Manifest::new("demo", mgc_types::ecosystem::Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("react").unwrap(),
            mgc_types::VersionRange::parse("^19.3.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let react_19 = PackageId::new(
        PackageName::new("react").unwrap(),
        Version::parse("19.3.0").unwrap(),
    );
    let react_18_peer = PackageId::new(
        PackageName::new("react").unwrap(),
        Version::parse("18.3.1").unwrap(),
    );

    assert!(is_manifest_root_package(&manifest, &react_19));
    assert!(!is_manifest_root_package(&manifest, &react_18_peer));
}

#[test]
fn web_root_normalization_collapses_duplicate_direct_versions() {
    let mut manifest = Manifest::new("demo", mgc_types::ecosystem::Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("postcss").unwrap(),
            mgc_types::VersionRange::parse("^8.5.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("plugin").unwrap(),
            mgc_types::VersionRange::parse("^1.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );

    let older = PackageId::parse("postcss@8.5.23").unwrap();
    let newer = PackageId::parse("postcss@8.5.28").unwrap();
    let plugin = PackageId::parse("plugin@1.0.0").unwrap();
    let package = |id: PackageId, direct: bool, deps: Vec<PackageId>| ResolvedPackage {
        id,
        integrity: String::new(),
        tarball_url: String::new(),
        deps,
        peer_deps: vec![],
        direct,
        dev: false,
    };
    let graph = normalize_resolved_graph(
        &manifest,
        ResolvedGraph {
            packages: vec![
                package(older.clone(), true, vec![]),
                package(newer.clone(), true, vec![]),
                package(plugin.clone(), true, vec![older.clone()]),
            ],
        },
    );
    let packages: std::collections::HashMap<_, _> = graph
        .packages
        .iter()
        .map(|package| (package.id.clone(), package))
        .collect();

    assert!(
        packages.contains_key(&older),
        "the transitive version stays installed"
    );
    assert!(
        packages[&newer].direct,
        "only the latest matching root stays direct"
    );
    assert!(!packages[&older].direct);
    let root_ids: Vec<_> = select_root_packages(&graph)
        .into_iter()
        .filter(|package| package.id.name_str() == "postcss")
        .map(|package| package.id.clone())
        .collect();
    assert_eq!(root_ids, vec![newer]);
}
