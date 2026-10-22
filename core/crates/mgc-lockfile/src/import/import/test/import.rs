//! Tests source lockfile attribution for imported rows.
//! Kiểm tra ghi nhận lockfile nguồn cho row được import.

use crate::{
    EcosystemTag, SOURCE_KIND_REGISTRY_IMPORT, import_file, verify_ownership_completeness,
};

#[test]
fn imported_packages_keep_source_file_and_pass_classified_gate() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let path = directory.path().join("bun.lock");
    std::fs::write(
        &path,
        r#"{"lockfileVersion":1,"packages":{"foo@1.0.0":["foo@1.0.0"]}}"#,
    )
    .expect("test operation must succeed");

    let (mut lockfile, report) = import_file(&path).expect("test operation must succeed");
    assert_eq!(report.source_file, "bun.lock");
    assert_eq!(lockfile.packages.len(), 1);
    let package = &mut lockfile.packages[0];
    assert_eq!(
        package
            .provenance
            .as_ref()
            .map(|provenance| provenance.source_kind.as_str()),
        Some(SOURCE_KIND_REGISTRY_IMPORT)
    );
    assert_eq!(
        package
            .provenance
            .as_ref()
            .and_then(|provenance| provenance.imported_from.as_deref()),
        Some("bun.lock")
    );

    package.ecosystem = EcosystemTag::Web;
    assert!(verify_ownership_completeness(&lockfile).is_ok());
}
