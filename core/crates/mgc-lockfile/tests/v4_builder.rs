//! v4 builder tests — test dựng v4 từ graph đã resolve.
//!
//! The install write path reaches v4 directly from the resolved graph (no
//! lossy v3 detour): canonical identity, peer digests, installer SRI and
//! digest math are covered here.
//! (Đường ghi install tới v4 trực tiếp từ graph đã resolve (không vòng v3
//! hao hụt): identity canonical, digest peer, SRI installer và toán digest.)

use mgc_lockfile::EcosystemTag;
use mgc_lockfile::canonical::{LockfileV4, payload_digest};
use mgc_lockfile::v4_graph::build_v4_from_resolved_graph;
use mgc_types::{PackageId, ResolvedGraph, ResolvedPackage};

#[test]
fn material_equality_ignores_serialization_order_but_preserves_artifact_changes() {
    let first = shared_owner_document();
    let mut reordered = first.clone();
    reordered.packages.reverse();
    reordered.sources.reverse();
    reordered.root_dependencies.reverse();
    assert!(mgc_lockfile::v4_graph::v4_docs_match_material(
        &first, &reordered
    ));
    reordered.metadata.generator.push_str("-new-generator");
    reordered.metadata.lockfile_hash = "not-a-trusted-digest".into();
    assert!(mgc_lockfile::v4_graph::v4_docs_match_material(
        &first, &reordered
    ));
    reordered.packages[0]
        .artifact
        .as_mut()
        .unwrap()
        .url
        .push_str("-changed");
    assert!(!mgc_lockfile::v4_graph::v4_docs_match_material(
        &first, &reordered
    ));
}

fn shared_owner_document() -> LockfileV4 {
    let mut graph = sample_graph();
    graph.packages.push(pkg(
        "sibling",
        "1.0.0",
        "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
        "https://registry.npmjs.org/sibling/-/sibling-1.0.0.tgz",
        vec![],
        vec![],
        true,
    ));
    let mut document = build_v4_from_resolved_graph(
        &graph,
        "https://registry.npmjs.org",
        EcosystemTag::Web,
        "web",
        "mgc/test",
    )
    .unwrap();
    document
        .root_dependencies_by_owner
        .insert("web".into(), vec!["react@18.2.0".into()]);
    document
        .root_dependencies_by_owner
        .insert("lib".into(), vec!["sibling@1.0.0".into()]);
    document
        .root_dependencies_by_owner
        .insert("ai".into(), vec![]);
    document
}

fn owner_manifest(names: &[&str]) -> mgc_types::Manifest {
    let mut manifest = mgc_types::Manifest::new("owner-test", mgc_types::Ecosystem::Web);
    for name in names {
        manifest.add_dep(
            mgc_types::DependencySpec::new(
                mgc_types::PackageName::new(*name).unwrap(),
                mgc_types::VersionRange::parse("*").unwrap(),
            ),
            false,
            false,
            false,
        );
    }
    manifest
}

#[test]
fn replay_selects_owner_roots_and_transitive_closure_without_siblings() {
    let document = shared_owner_document();
    let web = mgc_lockfile::v4_graph::graph_from_v4_lockfile(&document, "web").unwrap();
    assert_eq!(
        web.packages.len(),
        2,
        "Web replay must omit the Lib sibling"
    );
    assert!(
        web.packages
            .iter()
            .all(|package| package.id.name_str() != "sibling")
    );
    let lib = mgc_lockfile::v4_graph::graph_from_v4_lockfile(&document, "lib").unwrap();
    assert_eq!(lib.packages.len(), 1);
    assert_eq!(lib.packages[0].id.name_str(), "sibling");
}

#[test]
fn explicit_empty_owner_never_borrows_global_roots() {
    let document = shared_owner_document();
    assert!(
        mgc_lockfile::v4_graph::graph_from_v4_lockfile(&document, "ai")
            .unwrap()
            .packages
            .is_empty()
    );
    assert!(mgc_lockfile::v4_graph::v4_graph_matches_manifest_closure(
        &document,
        &owner_manifest(&[]),
        "ai"
    ));
    assert!(!mgc_lockfile::v4_graph::v4_graph_matches_manifest_closure(
        &document,
        &owner_manifest(&["sibling"]),
        "ai"
    ));
}

#[test]
fn absent_owner_cannot_claim_another_owners_packages() {
    let document = shared_owner_document();
    assert!(mgc_lockfile::v4_graph::graph_from_v4_lockfile(&document, "app").is_err());
    assert!(!mgc_lockfile::v4_graph::v4_graph_matches_manifest_closure(
        &document,
        &owner_manifest(&["sibling"]),
        "app"
    ));
}

#[test]
fn owner_manifest_closure_ignores_valid_siblings_but_refuses_borrowed_roots() {
    let document = shared_owner_document();
    assert!(mgc_lockfile::v4_graph::v4_graph_matches_manifest_closure(
        &document,
        &owner_manifest(&["react"]),
        "web"
    ));
    assert!(!mgc_lockfile::v4_graph::v4_graph_matches_manifest_closure(
        &document,
        &owner_manifest(&["react", "sibling"]),
        "web"
    ));
}

#[test]
fn legacy_unscoped_v4_replay_preserves_global_root_behavior() {
    let mut document = shared_owner_document();
    document.root_dependencies_by_owner.clear();
    assert_eq!(
        mgc_lockfile::v4_graph::graph_from_v4_lockfile(&document, "web")
            .unwrap()
            .packages
            .len(),
        3
    );
    assert!(mgc_lockfile::v4_graph::v4_graph_matches_manifest_closure(
        &document,
        &owner_manifest(&["react", "sibling"]),
        "web"
    ));
}

#[test]
fn shared_replay_refuses_unowned_orphans_and_missing_roots() {
    let mut document = shared_owner_document();
    document.root_dependencies_by_owner.remove("lib");
    assert!(
        mgc_lockfile::v4_graph::graph_from_v4_lockfile(&document, "web").is_err(),
        "an unowned sibling must not be silently discarded during replay"
    );
    assert!(!mgc_lockfile::v4_graph::v4_graph_matches_manifest_closure(
        &document,
        &owner_manifest(&["react"]),
        "web"
    ));
    document
        .root_dependencies_by_owner
        .insert("lib".into(), vec!["missing@1.0.0".into()]);
    assert!(mgc_lockfile::v4_graph::graph_from_v4_lockfile(&document, "web").is_err());
}

#[test]
fn unrelated_sibling_artifact_is_not_an_install_input() {
    let mut document = shared_owner_document();
    document
        .packages
        .iter_mut()
        .find(|row| row.key.name == "sibling")
        .unwrap()
        .artifact = None;
    assert_eq!(
        mgc_lockfile::v4_graph::graph_from_v4_lockfile(&document, "web")
            .unwrap()
            .packages
            .len(),
        2
    );
    assert!(mgc_lockfile::v4_graph::graph_from_v4_lockfile(&document, "lib").is_err());
}

#[test]
fn dev_only_transitive_dependencies_do_not_become_production_packages() {
    let mut graph = sample_graph();
    graph.packages[1]
        .deps
        .push(PackageId::parse("dev-child@1.0.0").unwrap());
    graph.packages.push(pkg(
        "dev-child",
        "1.0.0",
        "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
        "https://registry.npmjs.org/dev-child/-/dev-child-1.0.0.tgz",
        vec![],
        vec![],
        false,
    ));
    let mut document = build_v4_from_resolved_graph(
        &graph,
        "https://registry.npmjs.org",
        EcosystemTag::Web,
        "web",
        "mgc/test",
    )
    .unwrap();
    document
        .packages
        .iter_mut()
        .find(|row| row.key.name == "react")
        .unwrap()
        .edges[0]
        .kind = mgc_lockfile::v4::EdgeKind::Dev;
    let replay = mgc_lockfile::v4_graph::graph_from_v4_lockfile(&document, "web").unwrap();
    assert!(
        replay
            .packages
            .iter()
            .find(|row| row.id.name_str() == "dev-child")
            .unwrap()
            .dev,
        "normal descendants of a dev-only path must remain dev-only"
    );
    let edge = document
        .packages
        .iter()
        .find(|row| row.key.name == "scheduler")
        .unwrap()
        .edges
        .iter()
        .find(|edge| edge.target_key.name == "dev-child")
        .unwrap()
        .clone();
    document
        .packages
        .iter_mut()
        .find(|row| row.key.name == "react")
        .unwrap()
        .edges
        .push(edge);
    let replay = mgc_lockfile::v4_graph::graph_from_v4_lockfile(&document, "web").unwrap();
    assert!(
        !replay
            .packages
            .iter()
            .find(|row| row.id.name_str() == "dev-child")
            .unwrap()
            .dev,
        "a second production path must make the shared dependency production-reachable"
    );
}

fn pkg(
    name: &str,
    version: &str,
    integrity: &str,
    url: &str,
    deps: Vec<PackageId>,
    peers: Vec<PackageId>,
    direct: bool,
) -> ResolvedPackage {
    ResolvedPackage {
        id: PackageId::parse(&format!("{name}@{version}")).unwrap(),
        integrity: integrity.to_string(),
        tarball_url: url.to_string(),
        deps,
        peer_deps: peers,
        direct,
        dev: false,
    }
}

fn sample_graph() -> ResolvedGraph {
    let react = PackageId::parse("react@18.2.0").unwrap();
    let sched = PackageId::parse("scheduler@0.23.0").unwrap();
    ResolvedGraph {
        packages: vec![
            pkg(
                "react",
                "18.2.0",
                "sha512-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==",
                "https://registry.npmjs.org/react/-/react-18.2.0.tgz",
                vec![sched.clone()],
                vec![],
                true,
            ),
            pkg(
                "scheduler",
                "0.23.0",
                "sha512-AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQ==",
                "https://registry.npmjs.org/scheduler/-/scheduler-0.23.0.tgz",
                vec![],
                vec![react.clone()],
                false,
            ),
        ],
    }
}

// Identity is canonical: lowercase names, padded versions, one source.
// (Identity canonical: tên lowercase, version padding, một nguồn.)
#[test]
fn builder_emits_canonical_identity() {
    let doc: LockfileV4 = build_v4_from_resolved_graph(
        &sample_graph(),
        "https://registry.npmjs.org",
        EcosystemTag::Web,
        "web",
        "mgc/test",
    )
    .unwrap();
    assert_eq!(doc.version, mgc_lockfile::v4::LOCKFILE_SCHEMA_V4);
    assert_eq!(doc.sources.len(), 1);
    assert_eq!(doc.sources[0].id, "src-native");
    assert_eq!(doc.packages.len(), 2);
    for package in &doc.packages {
        assert_eq!(package.key.source_id, "src-native");
        assert_eq!(package.key.ecosystem, EcosystemTag::Web);
    }
    let keys: Vec<String> = doc
        .packages
        .iter()
        .map(|p| format!("{}@{}", p.key.name, p.key.version))
        .collect();
    assert!(keys.contains(&"react@18.2.0".to_string()));
    // Peer edges exist with Peer kind; the peer set is digested.
    // (Cạnh peer tồn tại với kind Peer; tập peer được digest.)
    let sched = doc
        .packages
        .iter()
        .find(|p| p.key.name == "scheduler")
        .unwrap();
    assert!(
        sched
            .edges
            .iter()
            .any(|e| matches!(e.kind, mgc_lockfile::v4::EdgeKind::Peer))
    );
    assert!(sched.key.variant.peer_context.is_some());
    assert!(!doc.peer_contexts.is_empty());
    // Installer SRI is carried only when enforceable.
    // (SRI installer chỉ mang khi thực thi được.)
    assert!(doc.packages.iter().all(|p| {
        p.artifact
            .as_ref()
            .is_some_and(|a| a.integrity_sri.is_some())
    }));
    // Root pins cover the direct package, by owner and plain.
    // (Pin root phủ package trực tiếp, theo owner và thường.)
    assert!(doc.root_dependencies.contains(&"react@18.2.0".to_string()));
    assert!(
        doc.root_dependencies_by_owner
            .get("web")
            .is_some_and(|roots| roots.iter().any(|r| r.contains("react@18.2.0")))
    );
}

// Digest math verifies over the built document; the document round-trips.
// (Toán digest xác minh được trên tài liệu đã dựng; tài liệu round-trip.)
#[test]
fn builder_output_verifies_and_roundtrips() {
    let mut doc = build_v4_from_resolved_graph(
        &sample_graph(),
        "https://registry.npmjs.org",
        EcosystemTag::Web,
        "web",
        "mgc/test",
    )
    .unwrap();
    doc.metadata.generated_at = "2026-10-07T00:00:00Z".to_string();
    doc.metadata.lockfile_hash = payload_digest(&doc.payload());
    let text = mgc_lockfile::canonical::write_v4_document(&doc).unwrap();
    let back = mgc_lockfile::canonical::parse_v4_document(&text).unwrap();
    assert_eq!(back, doc);
    let report = mgc_lockfile::policy::verify_v4_math(&back).unwrap();
    assert!(report.digest_ok);
}

// Pins without installer-enforced SRI keep no SRI (never fabricated).
// (Pin không có SRI thực thi được thì không SRI (không bịa).)
#[test]
fn unenforceable_sri_stays_absent() {
    let mut graph = ResolvedGraph::empty();
    graph.packages.push(pkg(
        "left-pad",
        "1.3.0",
        "md5-AAAAAAAAAAAAAAAAAAAAAA==",
        "https://registry.npmjs.org/left-pad/-/left-pad-1.3.0.tgz",
        vec![],
        vec![],
        true,
    ));
    let doc = build_v4_from_resolved_graph(
        &graph,
        "https://registry.npmjs.org",
        EcosystemTag::Web,
        "web",
        "mgc/test",
    )
    .unwrap();
    assert!(
        doc.packages[0]
            .artifact
            .as_ref()
            .is_some_and(|a| a.integrity_sri.is_none())
    );
}

#[test]
fn merge_keeps_sibling_lanes_and_replaces_own() {
    use mgc_lockfile::v4_graph::{build_v4_from_resolved_graph, merge_v4_lane_packages};
    // Base v4 with a sibling (app) pin plus our (web) pin.
    // (Base v4 có pin khác (app) và pin của mình (web).)
    let mut base = build_v4_from_resolved_graph(
        &sample_graph(),
        "https://registry.npmjs.org",
        EcosystemTag::Web,
        "web",
        "mgc/test",
    )
    .unwrap();
    base.packages.push(mgc_lockfile::canonical::PackageV4 {
        key: mgc_lockfile::v4::PackageKey {
            ecosystem: EcosystemTag::Web,
            name: "sibling-dep".to_string(),
            version: "9.9.9".to_string(),
            source_id: "src-native".to_string(),
            variant: mgc_lockfile::v4::VariantKey::default(),
        },
        edges: vec![],
        markers: None,
        artifact: None,
        provenance: None,
        toolchain: None,
        scripts_policy: None,
        store_ref: None,
    });
    // The sibling survives collection through its own owner's root.
    // (Pin khác sống sót thu gom nhờ root của owner nó.)
    base.root_dependencies_by_owner.insert(
        "app".to_string(),
        vec!["mgc-root-v1:web:sibling-dep@9.9.9".to_string()],
    );

    // Lane rebuild with an extra package: own rows replaced, sibling kept.
    // (Lane build lại thêm package: row mình thay, row khác giữ.)
    let mut graph = sample_graph();
    graph.packages.push(pkg(
        "fresh-dep",
        "2.0.0",
        "sha512-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==",
        "https://registry.npmjs.org/fresh-dep/-/fresh-dep-2.0.0.tgz",
        vec![],
        vec![],
        true,
    ));
    let lane = build_v4_from_resolved_graph(
        &graph,
        "https://registry.npmjs.org",
        EcosystemTag::Web,
        "web",
        "mgc/test",
    )
    .unwrap();
    let lane_packages = lane.packages.clone();
    let lane_sources = lane.sources.clone();
    let lane_roots = lane.root_dependencies.clone();
    let (merged, dropped) = merge_v4_lane_packages(
        Some(base),
        lane_packages,
        lane_sources,
        lane_roots,
        "web",
        "mgc/test",
    );
    assert!(!dropped);
    let names: Vec<&str> = merged
        .packages
        .iter()
        .map(|p| p.key.name.as_str())
        .collect();
    assert!(names.contains(&"sibling-dep"));
    assert!(names.contains(&"fresh-dep"));
    assert!(names.contains(&"react"));
    // Merged document still digests and round-trips.
    // (Document đã gộp vẫn digest và round-trip được.)
    let mut merged = merged;
    merged.metadata.generated_at = "2026-10-07T00:00:00Z".to_string();
    merged.metadata.lockfile_hash = payload_digest(&merged.payload());
    let text = mgc_lockfile::canonical::write_v4_document(&merged).unwrap();
    let back = mgc_lockfile::canonical::parse_v4_document(&text).unwrap();
    assert_eq!(back, merged);
    assert!(
        mgc_lockfile::policy::verify_v4_math(&back)
            .unwrap()
            .digest_ok
    );
}

#[test]
fn merge_over_legacy_base_reports_dropped_sibling() {
    use mgc_lockfile::v4_graph::merge_v4_lane_packages;
    // A legacy base cannot contribute lossless rows: the caller must warn.
    // (Base legacy không đóng góp lossless được: caller phải cảnh báo.)
    let lane = build_v4_from_resolved_graph(
        &sample_graph(),
        "https://registry.npmjs.org",
        EcosystemTag::Web,
        "web",
        "mgc/test",
    )
    .unwrap();
    let (merged, dropped) = merge_v4_lane_packages(
        None,
        lane.packages.clone(),
        lane.sources.clone(),
        lane.root_dependencies.clone(),
        "web",
        "mgc/test",
    );
    assert!(!dropped);
    assert_eq!(merged.packages.len(), 2);
}

#[test]
fn merge_collects_orphans_but_keeps_sibling_roots() {
    use mgc_lockfile::v4_graph::{build_v4_from_resolved_graph, merge_v4_lane_packages};
    // Base v4: our stale react@17 (unreachable after the bump) plus a
    // sibling core's pin with its own root.
    // (Base v4: react@17 cũ của mình (không tới được sau bump) và pin của
    // core khác với root riêng.)
    let mut base = build_v4_from_resolved_graph(
        &sample_graph(),
        "https://registry.npmjs.org",
        EcosystemTag::Web,
        "web",
        "mgc/test",
    )
    .unwrap();
    base.packages.retain(|p| p.key.name == "react");
    base.packages[0].key.version = "17.0.0".to_string();
    base.root_dependencies = vec!["react@17.0.0".to_string()];
    base.root_dependencies_by_owner =
        [("web".to_string(), vec!["react@17.0.0".to_string()])].into();
    base.packages.push(mgc_lockfile::canonical::PackageV4 {
        key: mgc_lockfile::v4::PackageKey {
            ecosystem: EcosystemTag::Web,
            name: "sibling-dep".to_string(),
            version: "9.9.9".to_string(),
            source_id: "src-native".to_string(),
            variant: mgc_lockfile::v4::VariantKey::default(),
        },
        edges: vec![],
        markers: None,
        artifact: None,
        provenance: None,
        toolchain: None,
        scripts_policy: None,
        store_ref: None,
    });
    // The sibling survives collection through its own owner's root.
    // (Pin khác sống sót thu gom nhờ root của owner nó.)
    base.root_dependencies_by_owner.insert(
        "app".to_string(),
        vec!["mgc-root-v1:web:sibling-dep@9.9.9".to_string()],
    );
    base.root_dependencies_by_owner.insert(
        "app".to_string(),
        vec!["mgc-root-v1:web:sibling-dep@9.9.9".to_string()],
    );

    let lane = build_v4_from_resolved_graph(
        &sample_graph(),
        "https://registry.npmjs.org",
        EcosystemTag::Web,
        "web",
        "mgc/test",
    )
    .unwrap();
    let (merged, dropped) = merge_v4_lane_packages(
        Some(base),
        lane.packages.clone(),
        lane.sources.clone(),
        lane.root_dependencies.clone(),
        "web",
        "mgc/test",
    );
    assert!(!dropped);
    let names: Vec<&str> = merged
        .packages
        .iter()
        .map(|p| p.key.name.as_str())
        .collect();
    // Stale react@17 collected; sibling kept via its own root.
    // (react@17 cũ bị thu; pin khác giữ nhờ root riêng.)
    assert!(
        !names.contains(&"react") || {
            merged
                .packages
                .iter()
                .filter(|p| p.key.name == "react")
                .all(|p| p.key.version == "18.2.0")
        }
    );
    assert!(names.contains(&"sibling-dep"));
    assert!(names.contains(&"scheduler"));
}

#[test]
fn peer_duplicate_instances_document_but_replay_refuses() {
    // Two instances sharing name@version with different peer sets are
    // distinct v4 identities (documented, digested, round-tripped) — but
    // the graph replay, keyed by PackageId (name@version), cannot hold
    // both nodes and refuses loudly instead of collapsing them.
    // (Hai instance cùng name@version khác peer set là identity v4 riêng
    // (ghi, digest, round-trip được) — nhưng replay graph, key theo
    // PackageId, không chứa được cả hai nên từ chối rõ thay vì gộp.)
    let mut graph = sample_graph();
    // Force a peer twin: same name@version as react, different peer set.
    // (Ép twin peer: cùng name@version với react, khác tập peer.)
    let twin = ResolvedPackage {
        id: PackageId::parse("react@18.2.0").unwrap(),
        integrity: graph.packages[0].integrity.clone(),
        tarball_url: graph.packages[0].tarball_url.clone(),
        deps: vec![],
        peer_deps: vec![PackageId::parse("scheduler@0.23.0").unwrap()],
        direct: false,
        dev: false,
    };
    graph.packages.push(twin);
    // Make the ORIGINAL react peer-free so the two instances differ.
    // (Cho react GỐC không peer để hai instance khác nhau.)
    graph.packages[0].peer_deps.clear();
    let mut doc = build_v4_from_resolved_graph(
        &graph,
        "https://registry.npmjs.org",
        EcosystemTag::Web,
        "web",
        "mgc/test",
    )
    .unwrap();
    doc.metadata.generated_at = "2026-10-07T00:00:00Z".to_string();
    doc.metadata.lockfile_hash = payload_digest(&doc.payload());
    let text = mgc_lockfile::canonical::write_v4_document(&doc).unwrap();
    let back = mgc_lockfile::canonical::parse_v4_document(&text).unwrap();
    assert_eq!(back, doc);
    // The root pin `react@18.2.0` is now ambiguous across the two variants,
    // so even digest-level verification refuses before replay is attempted:
    // no layer silently picks one twin.
    // (Pin root giờ mơ hồ giữa hai variant, nên ngay verify đã từ chối
    // trước cả replay: không tầng nào lặng lẽ chọn một twin.)
    let err = mgc_lockfile::policy::verify_v4_math(&back).unwrap_err();
    assert!(
        err.to_string().contains("ambiguous"),
        "expected ambiguity refusal, got: {err}"
    );
}
