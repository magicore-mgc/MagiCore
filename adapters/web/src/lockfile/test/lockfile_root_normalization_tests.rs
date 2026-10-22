// lockfile_root_normalization_tests — root normalization.
// lockfile_root_normalization_tests — kiểm thử root normalization.
use super::*;

#[test]
fn web_root_normalization_preserves_a_unique_locked_root_version() {
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
                package(newer.clone(), false, vec![]),
                package(plugin.clone(), true, vec![newer.clone()]),
            ],
        },
    );
    let packages: std::collections::HashMap<_, _> = graph
        .packages
        .iter()
        .map(|package| (package.id.clone(), package))
        .collect();

    assert!(
        packages[&older].direct,
        "a unique lock root remains selected"
    );
    assert!(
        !packages[&newer].direct,
        "the matching transitive version stays nested"
    );
    let root_ids: Vec<_> = select_root_packages(&graph)
        .into_iter()
        .filter(|package| package.id.name_str() == "postcss")
        .map(|package| package.id.clone())
        .collect();
    assert_eq!(root_ids, vec![older]);
}
