//! Verify graph integrity failures map to the shared integrity category.
//! Kiểm tra lỗi integrity graph được ánh xạ sang category integrity dùng chung.

use super::*;

#[test]
fn duplicate_package_maps_to_integrity_error() {
    let error = MgError::from(FlutterPackageGraphError::DuplicatePackage(
        "shared".to_string(),
    ));

    assert!(matches!(
        error,
        MgError::Integrity(message) if message.contains("duplicate package name 'shared'")
    ));
}

#[test]
fn source_error_keeps_its_cause_after_shared_mapping() {
    let source = std::io::Error::other("source marker");
    let error = MgError::from(FlutterPackageGraphError::ReadRootPubspec(source));

    let MgError::Anyhow(error) = error else {
        panic!("root pubspec I/O error lost its typed source chain");
    };
    assert_eq!(
        error.to_string(),
        "read Flutter pubspec for package graph: source marker"
    );
    assert!(
        error
            .chain()
            .any(|cause| cause.to_string() == "source marker")
    );
}
