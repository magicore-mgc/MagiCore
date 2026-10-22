// lockfile_v4_merge_tests — v4 owner merge and orphan recovery.
// lockfile_v4_merge_tests — kiểm thử v4 owner merge and orphan recovery.
use super::*;

#[test]
fn web_v4_writer_adds_new_owner_without_replaying_sibling_packages() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"web\"\necosystem = \"web\"\n[lock]\nschema = \"v4\"\n",
    )
    .unwrap();
    let graph_for = |name: &str| ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: PackageId::parse(&format!("{name}@1.0.0")).unwrap(),
            integrity: "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".into(),
            tarball_url: format!("https://registry.example/{name}.tgz"),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };
    let mut base = mgc_lockfile::v4_graph::build_v4_from_resolved_graph(
        &graph_for("lib-only"),
        "https://registry.example",
        mgc_lockfile::EcosystemTag::Web,
        "lib",
        "mgc/test",
    )
    .unwrap();
    let bytes = mgc_lockfile::writer::finalize_v4_document(&mut base).unwrap();
    std::fs::write(dir.path().join("mgc.lock"), bytes).unwrap();
    let web = graph_for("web-only");
    crate::lockfile::write_web_lockfile_with_state(dir.path(), &web, "https://registry.example")
        .unwrap();
    let bytes = std::fs::read(dir.path().join("mgc.lock")).unwrap();
    let doc =
        mgc_lockfile::canonical::parse_v4_document(std::str::from_utf8(&bytes).unwrap()).unwrap();
    assert_eq!(doc.packages.len(), 2);
    assert!(doc.root_dependencies_by_owner.contains_key("lib"));
    assert!(doc.root_dependencies_by_owner.contains_key("web"));
    crate::lockfile::write_web_lockfile_with_state(dir.path(), &web, "https://registry.example")
        .unwrap();
    assert_eq!(std::fs::read(dir.path().join("mgc.lock")).unwrap(), bytes);
}

#[test]
fn web_v4_fresh_writer_repairs_orphan_without_losing_sibling_owner() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"web\"\necosystem = \"web\"\n[lock]\nschema = \"v4\"\n",
    )
    .unwrap();
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: PackageId::parse("web-only@1.0.0").unwrap(),
            integrity: "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".into(),
            tarball_url: "https://registry.example/web-only.tgz".into(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };
    let mut base = mgc_lockfile::v4_graph::build_v4_from_resolved_graph(
        &graph,
        "https://registry.example",
        mgc_lockfile::EcosystemTag::Web,
        "web",
        "mgc/test",
    )
    .unwrap();
    let mut sibling = base.packages[0].clone();
    sibling.key.name = "lib-only".into();
    base.packages.push(sibling);
    base.root_dependencies_by_owner
        .insert("lib".into(), vec!["lib-only@1.0.0".into()]);
    let mut orphan = base.packages[0].clone();
    orphan.key.name = "orphan".into();
    base.packages.push(orphan);
    let bytes = mgc_lockfile::writer::finalize_v4_document(&mut base).unwrap();
    std::fs::write(dir.path().join("mgc.lock"), bytes).unwrap();
    crate::lockfile::write_web_lockfile_with_state(dir.path(), &graph, "https://registry.example")
        .unwrap();
    let repaired = mgc_lockfile::canonical::parse_v4_document(
        &std::fs::read_to_string(dir.path().join("mgc.lock")).unwrap(),
    )
    .unwrap();
    assert_eq!(repaired.packages.len(), 2);
    assert_eq!(
        repaired.root_dependencies_by_owner["lib"],
        base.root_dependencies_by_owner["lib"]
    );
    let replayed = mgc_lockfile::v4_graph::graph_from_v4_lockfile(&repaired, "web").unwrap();
    assert_eq!(replayed.packages.len(), 1);
    assert_eq!(replayed.packages[0].id.to_string(), "web-only@1.0.0");
}
