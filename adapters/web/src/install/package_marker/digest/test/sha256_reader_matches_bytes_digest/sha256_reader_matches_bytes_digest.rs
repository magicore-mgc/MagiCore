// Preserve parity between reader and byte digests — Giữ digest reader và bytes đồng nhất.
#[test]
fn sha256_reader_matches_bytes_digest() {
    assert_eq!(
        compute_sha256_hex_from_reader(std::io::Cursor::new(b"package archive")).unwrap(),
        compute_sha256_hex(b"package archive")
    );
}
