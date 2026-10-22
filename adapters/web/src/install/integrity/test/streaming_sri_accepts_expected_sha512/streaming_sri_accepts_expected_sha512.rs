// Check the cache-hit streaming integrity route — Kiểm tra route integrity stream của cache hit.
#[test]
fn streaming_sri_accepts_expected_sha512() {
    use mgc_types::{PackageId, adapter::ResolvedPackage};

    let bytes = b"verified streamed tarball";
    let package = ResolvedPackage {
        id: PackageId::parse("streamed-integrity@1.0.0").unwrap(),
        integrity: format!("sha512-{}", crate::lockfile::compute_sha512_b64(bytes)),
        tarball_url: String::new(),
        deps: Vec::new(),
        peer_deps: Vec::new(),
        direct: true,
        dev: false,
    };

    assert!(verify_tarball_reader_integrity(&package, std::io::Cursor::new(bytes)).is_ok());
}
