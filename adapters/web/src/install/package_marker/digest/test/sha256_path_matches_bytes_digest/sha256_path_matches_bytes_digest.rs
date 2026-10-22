// Preserve path and byte digest parity — Giữ kết quả digest path và bytes đồng nhất.
#[test]
fn sha256_path_matches_bytes_digest() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("artifact.tgz");
    std::fs::write(&path, b"package archive").unwrap();

    assert_eq!(
        compute_sha256_hex_from_path(&path).unwrap(),
        compute_sha256_hex(b"package archive")
    );
}
