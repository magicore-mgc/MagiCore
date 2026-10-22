// Reject a streamed cache payload with the wrong SRI — Từ chối payload cache stream sai SRI.
#[test]
fn streaming_sri_rejects_mismatched_sha512() {
    use mgc_types::{PackageId, adapter::ResolvedPackage};

    let package = ResolvedPackage {
        id: PackageId::parse("bad-integrity@1.0.0").unwrap(),
        integrity: "sha512-AAAAAAAA".to_string(),
        tarball_url: String::new(),
        deps: Vec::new(),
        peer_deps: Vec::new(),
        direct: true,
        dev: false,
    };

    assert!(
        verify_tarball_reader_integrity(&package, std::io::Cursor::new(b"different bytes"))
            .is_err()
    );
}
