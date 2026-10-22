#![allow(clippy::unwrap_used)]

use super::super::super::*;
use mgc_crypto::keyring::KeyPair;
use mgc_lockfile::{Lockfile, Package};
use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};
use tempfile::tempdir;

#[test]
fn test_load_locked_graph_rejects_unsupported_lock_version() {
    let dir = tempdir().unwrap();
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
    lock.version = "0".into(); // Unsupported version
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
    std::fs::write(
        dir.path().join("mgc.lock"),
        mgc_lockfile::serialization::to_toml(&lock).unwrap(),
    )
    .unwrap();

    let err = load_locked_graph(dir.path(), "web", &manifest).unwrap_err();
    assert!(err.to_string().contains("unsupported lockfile version"));
}

#[test]
fn test_load_locked_graph_ignores_legacy_checksum_sidecar() {
    let dir = tempdir().unwrap();
    let manifest = Manifest::new("demo", Ecosystem::Web);
    let mut lock = Lockfile::new();
    let lock_path = dir.path().join("mgc.lock");
    let key = KeyPair::generate().unwrap();
    mgc_lockfile::sign_and_write_lockfile(&mut lock, &lock_path, &key).unwrap();
    std::fs::write(dir.path().join("mgc.lock.sha256"), "bad").unwrap();
    std::fs::write(
        dir.path().join("mgc.toml"),
        format!(
            "name = \"demo\"\necosystem = \"web\"\n[lock]\npolicy = \"require\"\n[trust]\nkeys = [\"{}\"]\n",
            key.key_id
        ),
    )
    .unwrap();

    let graph = load_locked_graph(dir.path(), "web", &manifest)
        .unwrap()
        .expect("a signed empty lock is a valid graph for an empty manifest");
    assert!(graph.packages.is_empty());
}

#[test]
fn test_graph_from_lockfile_rejects_invalid_dependency_id() {
    let mut lock = Lockfile::new();
    lock.packages.push(Package {
        name: "react".into(),
        version: "18.2.0".into(),
        resolved: "https://registry.npmjs.org/react/-/react-18.2.0.tgz".into(),
        integrity: "sha256-test".into(),
        dependencies: vec!["not-a-package-id".into()],
        ecosystem: mgc_lockfile::EcosystemTag::Web,
        registry: Some("npm://https://registry.npmjs.org".into()),
        ..Package::default()
    });

    let err = graph_from_lockfile(&lock).unwrap_err();

    assert!(
        err.to_string().contains("invalid package spec"),
        "unexpected error: {err}"
    );
}
