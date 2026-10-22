//! Tests for lock loading and core-owner isolation.
//! Test nạp lock và cô lập package theo core owner.

use super::super::load_locked_graph;
use mgc_lockfile::{Lockfile, Package};
use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

#[test]
fn locked_graph_excludes_sibling_core_packages_with_same_ecosystem_and_name() {
    let temp = tempfile::tempdir().expect("tempdir");
    mgc_config::project::ProjectConfig::write_core_marker_at(temp.path(), "ai").unwrap();
    let mut lock = Lockfile::new();
    let mut ai = Package::new(
        "shared-python".into(),
        "2.1.0".into(),
        "https://pypi.org/ai.whl".into(),
        "sha256-ai".into(),
    );
    ai.ecosystem = mgc_lockfile::EcosystemTag::Python;
    ai.owner_core = Some("ai".into());
    ai.registry = Some("https://pypi.org/simple".into());
    let mut lib = Package::new(
        "shared-python".into(),
        "1.8.0".into(),
        "https://pypi.org/lib.whl".into(),
        "sha256-lib".into(),
    );
    lib.ecosystem = mgc_lockfile::EcosystemTag::Python;
    lib.owner_core = Some("lib".into());
    lib.registry = Some("https://pypi.org/simple".into());
    lock.packages = vec![ai, lib];
    std::fs::write(
        temp.path().join("mgc.lock"),
        mgc_lockfile::writer::serialize_lockfile(&lock).unwrap(),
    )
    .unwrap();
    let mut manifest = Manifest::new("ai-project", Ecosystem::Ai);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("shared-python").unwrap(),
            VersionRange::parse("^2.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );

    let graph = load_locked_graph(temp.path(), "ai", &manifest)
        .unwrap()
        .expect("the lock should contain the AI-owned dependency");
    assert_eq!(graph.packages.len(), 1);
    assert_eq!(graph.packages[0].id.version().to_string(), "2.1.0");
}
