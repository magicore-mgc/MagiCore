// Use supplied integrity as the fast marker identity — Dùng integrity có sẵn làm identity marker nhanh.
#[test]
fn explicit_integrity_is_used_as_fast_fingerprint() {
    use mgc_types::adapter::ResolvedPackage;
    use mgc_types::{PackageId, PackageName, Version};

    let package = ResolvedPackage {
        id: PackageId::new(
            PackageName::new("example").unwrap(),
            Version::parse("1.0.0").unwrap(),
        ),
        integrity: "sha512-value".to_string(),
        tarball_url: "https://registry.example.test/example.tgz".to_string(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    };

    let marker = expected_extracted_package_marker_fast(&package, b"archive");

    assert_eq!(marker.schema_version, 3);
    assert_eq!(marker.tarball_sha256, "integrity:sha512-value");
    assert_eq!(marker.integrity.as_deref(), Some("sha512-value"));
    assert_eq!(marker.file_count, 0);
    assert!(marker.file_tree_sha256.is_empty());
}
