// lockfile_foreign_ecosystem_tests — foreign ecosystem preservation.
// lockfile_foreign_ecosystem_tests — kiểm thử foreign ecosystem preservation.
use super::*;

#[test]
fn web_lock_writer_preserves_foreign_ecosystems_and_reads_only_web_pins() {
    let dir = tempdir_real().unwrap();
    let path = dir.path().join("mgc.lock");
    let mut lock = Lockfile::new();
    lock.packages.push(mgc_lockfile::Package {
        name: "react".into(),
        version: "18.2.0".into(),
        ecosystem: mgc_lockfile::EcosystemTag::Dart,
        registry: Some("https://pub.dev".into()),
        ..Default::default()
    });
    lock.packages.push(mgc_lockfile::Package {
        name: "react".into(),
        version: "17.0.0".into(),
        ecosystem: mgc_lockfile::EcosystemTag::Web,
        registry: Some("https://registry.npmjs.org".into()),
        ..Default::default()
    });
    std::fs::write(
        &path,
        mgc_lockfile::writer::serialize_lockfile(&lock).unwrap(),
    )
    .unwrap();

    let react = PackageId::new(
        PackageName::new("react").unwrap(),
        Version::parse("19.0.0").unwrap(),
    );
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: react,
            integrity: "sha512-web".into(),
            tarball_url: String::new(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };

    write_web_lockfile_with_state(dir.path(), &graph, "https://registry.npmjs.org").unwrap();
    let written = mgc_lockfile::parser::load_lockfile(&path).unwrap();
    assert!(written.packages.iter().any(|p| {
        p.name == "react"
            && p.version == "18.2.0"
            && p.ecosystem == mgc_lockfile::EcosystemTag::Dart
    }));
    assert!(written.packages.iter().any(|p| {
        p.name == "react" && p.version == "19.0.0" && p.ecosystem == mgc_lockfile::EcosystemTag::Web
    }));
    assert!(!written.packages.iter().any(|p| {
        p.name == "react" && p.version == "17.0.0" && p.ecosystem == mgc_lockfile::EcosystemTag::Web
    }));

    assert!(web_lockfile_matches_graph(&written, &graph));
    let mut manifest = Manifest::new("demo", mgc_types::ecosystem::Ecosystem::Web);
    manifest.add_dep(
        mgc_types::DependencySpec::new(
            PackageName::new("react").unwrap(),
            mgc_types::VersionRange::parse("^19.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    assert!(lockfile_satisfies_manifest(&written, &manifest));
    let cached_graph = build_graph_from_lockfile(&written, &manifest, "web")
        .unwrap()
        .expect("the Web entry is available even when another ecosystem shares its name");
    assert_eq!(cached_graph.packages[0].id.version().to_string(), "19.0.0");
}
