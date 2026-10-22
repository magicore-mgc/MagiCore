// Leave the verified reader ready for extraction — Để reader đã xác minh sẵn sàng cho extraction.
#[test]
fn reader_marker_rewinds_missing_integrity_fingerprint() {
    use std::io::Read;
    use mgc_types::adapter::ResolvedPackage;
    use mgc_types::{PackageId, PackageName, Version};

    let package = ResolvedPackage {
        id: PackageId::new(
            PackageName::new("reader-example").unwrap(),
            Version::parse("1.0.0").unwrap(),
        ),
        integrity: String::new(),
        tarball_url: String::new(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    };
    let bytes = b"reader marker fixture";
    let mut reader = std::io::Cursor::new(bytes);

    let marker = expected_extracted_package_marker_fast_from_reader(&package, &mut reader).unwrap();
    let expected = expected_extracted_package_marker_fast(&package, bytes);
    let mut restored = Vec::new();
    reader.read_to_end(&mut restored).unwrap();

    assert_eq!(marker.tarball_sha256, expected.tarball_sha256);
    assert_eq!(restored, bytes);
}
