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

// ── Root pins from owner-scoped storage ─────────────────────────────────
// v3 locks write `[root_dependencies_by_owner]` and leave the flat
// `root_dependencies` vector empty. Reading only the flat field made every direct
// dependency look like an orphan, so `mgc why <direct>` said "nothing depends on
// it — likely a leftover pin" and advised pruning a dependency the user declared.
// (Lock v3 ghi `root_dependencies_by_owner`, để vector phẳng rỗng. Đọc mỗi vector
// khiến mọi dependency trực tiếp trông như mồ côi và khuyên prune.)

use mgc_lockfile::EcosystemTag;
use mgc_lockfile::root_pin::{format_root_pin, parse_root_pin};

/// A qualified root pin decodes to the bare package name.
/// (Root pin có ecosystem giải mã về tên package trần.)
#[test]
fn a_qualified_root_pin_decodes_to_the_package_name() {
    let pin = format_root_pin(EcosystemTag::Web, "nanoid@5.1.16");
    assert_eq!(pin, "mgc-root-v1:web:nanoid@5.1.16");
    assert_eq!(edge_name(parse_root_pin(&pin).package_id), "nanoid");
}

/// The same decoding works for every ecosystem the lock can carry, so no core
/// silently loses its direct dependencies.
/// (Giải mã đúng cho mọi ecosystem lock có thể chứa, để không core nào mất
/// dependency trực tiếp.)
#[test]
fn every_ecosystem_root_pin_decodes() {
    for (ecosystem, name) in [
        (EcosystemTag::Web, "react"),
        (EcosystemTag::Python, "attrs"),
        (EcosystemTag::Rust, "itoa"),
        (EcosystemTag::Go, "github.com/x/y"),
        (EcosystemTag::Maven, "org.slf4j:slf4j-api"),
    ] {
        let pin = format_root_pin(ecosystem, &format!("{name}@1.0.0"));
        assert_eq!(
            edge_name(parse_root_pin(&pin).package_id),
            name,
            "{ecosystem:?} root pin must decode to '{name}'"
        );
    }
}

/// A legacy unqualified root pin still decodes (no prefix to strip).
/// (Root pin cũ không có prefix vẫn giải mã được.)
#[test]
fn a_legacy_root_pin_still_decodes() {
    assert_eq!(
        edge_name(parse_root_pin("lodash@4.17.21").package_id),
        "lodash"
    );
}
