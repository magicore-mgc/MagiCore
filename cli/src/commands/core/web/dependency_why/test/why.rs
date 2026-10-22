//! Web lockfile explanation regression tests.
//! Test regression giải thích lockfile Web.

use super::super::implementation::{edge_name, why};

#[test]
fn edge_name_strips_versions_and_ranges_without_losing_scope() {
    assert_eq!(edge_name("leftpad"), "leftpad");
    assert_eq!(edge_name("leftpad@1.3.0"), "leftpad");
    assert_eq!(edge_name("leftpad@^1.3.0"), "leftpad");
    assert_eq!(edge_name("@scope/pkg"), "@scope/pkg");
    assert_eq!(edge_name("@scope/pkg@1.3.0"), "@scope/pkg");
    assert_eq!(edge_name("  pad-core@2.0.0  "), "pad-core");
}

#[tokio::test]
#[cfg(feature = "web")]
async fn why_rejects_v4_graph_with_a_dangling_root_pin() {
    let project = tempfile::tempdir().expect("tempdir");
    let mut lock = mgc_lockfile::canonical::LockfileV4::new("mgc/test");
    lock.packages.push(mgc_lockfile::canonical::PackageV4 {
        key: mgc_lockfile::PackageKey {
            ecosystem: mgc_lockfile::EcosystemTag::Web,
            name: "requests".to_string(),
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
    lock.root_dependencies.push("missing@1.0.0".to_string());
    lock.metadata.lockfile_hash = mgc_lockfile::canonical::payload_digest(&lock.payload());
    std::fs::write(
        project.path().join("mgc.lock"),
        mgc_lockfile::canonical::write_v4_document(&lock).unwrap(),
    )
    .unwrap();
    let adapter =
        mgc_web_adapter::WebAdapter::with_registry("https://registry.npmjs.org".to_string())
            .unwrap();

    let error = why(&adapter, project.path(), "requests")
        .await
        .expect_err("diagnostics must reject an internally inconsistent v4 graph");

    assert!(error.to_string().contains("root pin"), "{error:#}");
}
