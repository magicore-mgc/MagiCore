//! Tests for lossless digest conversion to App protocol entries.
//! Test chuyển digest không mất dữ liệu sang protocol entry App.

// Legacy and SRI encodings must reconstruct the same verified digest.
// Lock V4 đổi encoding không được làm mất hash hoặc phá native frozen install.
#[test]
fn native_entry_preserves_digest_across_legacy_and_sri() {
    let mut package = mgc_types::ResolvedPackage {
        id: mgc_types::PackageId::new(
            mgc_types::PackageName::new("fixture").unwrap(),
            mgc_types::Version::parse("1.0.0").unwrap(),
        ),
        integrity: String::new(),
        tarball_url: String::new(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    };
    for raw in [
        "ab".repeat(32),
        "AB".repeat(32),
        "q6urq6urq6urq6urq6urq6urq6urq6urq6urq6urq6s=".to_string(),
    ] {
        package.integrity = format!("sha256-{raw}");
        assert_eq!(super::entry_from_package(&package).sha256, "ab".repeat(32));
    }
    for bad in ["sha256-", "sha256-broken", "sha256-AAAA"] {
        package.integrity = bad.to_string();
        assert!(
            !super::entry_from_package(&package).sha256.is_empty(),
            "malformed SHA256 must not become missing digest"
        );
    }
}
