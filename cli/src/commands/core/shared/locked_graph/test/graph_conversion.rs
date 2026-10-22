//! Tests for legacy and v4 lock graph conversion.
//! Test chuyển lock legacy/v4 thành graph.

use super::super::{
    graph_from_lockfile, graph_from_v4_lockfile, v4_graph_matches_manifest_closure,
};
use mgc_lockfile::{Lockfile, Package};
use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, ResolvedPackage, VersionRange};

#[test]
fn v2_graph_preserves_dependency_edges() {
    let mut lock = Lockfile::new();
    let mut react = Package::new(
        "react".into(),
        "19.2.0".into(),
        "https://registry.example/react.tgz".into(),
        "blake3-react".into(),
    );
    react.add_dependency("scheduler@0.25.0".into());
    lock.add_package(react);
    lock.add_package(Package::new(
        "scheduler".into(),
        "0.25.0".into(),
        "https://registry.example/scheduler.tgz".into(),
        "blake3-scheduler".into(),
    ));

    let graph = graph_from_lockfile(&lock, "web").unwrap();
    assert_eq!(graph.packages.len(), 2);
    assert_eq!(graph.packages[0].deps[0].to_string(), "scheduler@0.25.0");
}

fn v4_sample_doc() -> mgc_lockfile::canonical::LockfileV4 {
    use mgc_lockfile::v4::{Edge, EdgeKind, EdgeOrigin};
    let mut doc = mgc_lockfile::canonical::LockfileV4::new("mgc/test");
    let pin = |name: &str, version: &str, edges: Vec<Edge>| {
        mgc_lockfile::canonical::PackageV4 {
        key: mgc_lockfile::PackageKey {
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            name: name.to_string(),
            version: version.to_string(),
            source_id: "npm".to_string(),
            variant: mgc_lockfile::VariantKey::default(),
        },
        edges,
        artifact: Some(mgc_lockfile::schema::ArtifactRef {
            url: format!("https://registry.npmjs.org/{name}/-/{name}-{version}.tgz"),
            size_bytes: None,
            content_hash: String::new(),
            downloaded_from: "https://registry.npmjs.org".to_string(),
            integrity_sri: Some(
                "sha512-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=="
                    .to_string(),
            ),
        }),
        markers: None,
        provenance: None,
        toolchain: None,
        scripts_policy: None,
        store_ref: None,
    }
    };
    let edge = |name: &str, version: &str, kind: EdgeKind| Edge {
        target_key: mgc_lockfile::PackageKey {
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            name: name.to_string(),
            version: version.to_string(),
            source_id: "npm".to_string(),
            variant: mgc_lockfile::VariantKey::default(),
        },
        range: "*".to_string(),
        kind,
        origin: EdgeOrigin::Manifest,
        marker: None,
    };
    doc.packages.push(pin(
        "app",
        "1.0.0",
        vec![
            edge("leftpad", "1.3.0", EdgeKind::Normal),
            edge("devonly", "2.0.0", EdgeKind::Dev),
        ],
    ));
    doc.packages.push(pin(
        "leftpad",
        "1.3.0",
        vec![edge("react", "18.2.0", EdgeKind::Peer)],
    ));
    doc.packages.push(pin("react", "18.2.0", Vec::new()));
    doc.packages.push(pin("devonly", "2.0.0", Vec::new()));
    doc.root_dependencies_by_owner
        .insert("web".to_string(), vec!["app@1.0.0".to_string()]);
    doc
}

fn v4_sample_manifest() -> Manifest {
    let mut manifest = Manifest::new("demo", Ecosystem::Web);
    // Only app is a declared root; other pins are its transitive closure.
    // Chỉ app là root đã khai báo; các pin khác thuộc closure transitive.
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("app").unwrap(),
            VersionRange::parse("=1.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    manifest
}

#[test]
fn v4_graph_maps_edges_peers_and_dev_without_flattening() {
    let doc = v4_sample_doc();
    assert!(v4_graph_matches_manifest_closure(
        &doc,
        &v4_sample_manifest(),
        "web"
    ));
    let graph = graph_from_v4_lockfile(&doc, "web").unwrap();
    assert_eq!(graph.packages.len(), 4);
    let by_id: std::collections::HashMap<String, &ResolvedPackage> = graph
        .packages
        .iter()
        .map(|package| (package.id.to_string(), package))
        .collect();
    let app = by_id["app@1.0.0"];
    assert!(app.direct && !app.dev);
    assert_eq!(app.deps.len(), 2);
    assert!(app.peer_deps.is_empty());
    assert!(app.integrity.starts_with("sha512-"));
    assert!(app.tarball_url.contains("app-1.0.0.tgz"));
    let leftpad = by_id["leftpad@1.3.0"];
    assert!(!leftpad.direct && !leftpad.dev);
    assert!(
        leftpad
            .peer_deps
            .iter()
            .any(|id| id.to_string() == "react@18.2.0")
    );
    let devonly = by_id["devonly@2.0.0"];
    assert!(devonly.dev, "dev-only reachable package must be flagged");
    assert!(!by_id["react@18.2.0"].dev);
}

#[test]
fn v4_graph_refuses_duplicate_instances_missing_sri_and_dangling_edges() {
    let mut doc = v4_sample_doc();
    // Duplicate name@version with a distinct source must not collapse.
    // Không gộp package trùng tên/version nhưng khác source.
    let mut twin = doc.packages[1].clone();
    twin.key.source_id = "mirror".to_string();
    doc.packages.push(twin);
    assert!(graph_from_v4_lockfile(&doc, "web").is_err());

    let mut doc = v4_sample_doc();
    doc.packages[1].artifact.as_mut().unwrap().integrity_sri = None;
    let error = graph_from_v4_lockfile(&doc, "web").unwrap_err();
    assert!(
        error.to_string().contains("installer-enforced SRI"),
        "{error:#}"
    );

    let mut doc = v4_sample_doc();
    doc.packages[1].artifact.as_mut().unwrap().url = String::new();
    assert!(graph_from_v4_lockfile(&doc, "web").is_err());

    let mut doc = v4_sample_doc();
    doc.packages[0].edges.push(mgc_lockfile::v4::Edge {
        target_key: mgc_lockfile::PackageKey {
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            name: "ghost".to_string(),
            version: "9.9.9".to_string(),
            source_id: "npm".to_string(),
            variant: mgc_lockfile::VariantKey::default(),
        },
        range: "*".to_string(),
        kind: mgc_lockfile::v4::EdgeKind::Normal,
        origin: mgc_lockfile::v4::EdgeOrigin::Manifest,
        marker: None,
    });
    assert!(graph_from_v4_lockfile(&doc, "web").is_err());
}

#[test]
fn v4_closure_supports_rootless_web_locks_with_unique_manifest_anchors() {
    // Web-style locks deliberately carry no roots: the manifest anchors
    // the graph (exactly-one match per dependency) instead.
    // (Lock kiểu web không mang roots: manifest làm neo.)
    let mut doc = v4_sample_doc();
    doc.root_dependencies_by_owner.clear();
    doc.root_dependencies.clear();
    let mut manifest = v4_sample_manifest();
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("leftpad").unwrap(),
            VersionRange::parse("^1.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    assert!(v4_graph_matches_manifest_closure(&doc, &manifest, "web"));
    // Ambiguity fails closed: two leftpad versions, one manifest range.
    // Từ chối mơ hồ: hai version leftpad cùng thỏa một range manifest.
    let mut twin = doc.packages[1].clone();
    twin.key.version = "1.4.0".to_string();
    doc.packages.push(twin);
    assert!(!v4_graph_matches_manifest_closure(&doc, &manifest, "web"));
}

#[test]
fn v4_closure_rejects_orphans_and_manifest_mismatch() {
    let mut doc = v4_sample_doc();
    doc.packages.push(mgc_lockfile::canonical::PackageV4 {
        key: mgc_lockfile::PackageKey {
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            name: "orphan".to_string(),
            version: "1.0.0".to_string(),
            source_id: "npm".to_string(),
            variant: mgc_lockfile::VariantKey::default(),
        },
        edges: Vec::new(),
        markers: None,
        artifact: None,
        provenance: None,
        toolchain: None,
        scripts_policy: None,
        store_ref: None,
    });
    assert!(!v4_graph_matches_manifest_closure(
        &doc,
        &v4_sample_manifest(),
        "web"
    ));

    let doc = v4_sample_doc();
    let mut manifest = Manifest::new("demo", Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("missing").unwrap(),
            VersionRange::parse("^1.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    assert!(!v4_graph_matches_manifest_closure(&doc, &manifest, "web"));
}
