// Keep streamed SRI byte-for-byte compatible — Giữ SRI stream tương thích từng byte.
#[test]
fn streaming_tarball_integrity_matches_buffered_integrity() {
    let bytes = b"streamed package tarball bytes";

    let buffered = compute_tarball_integrity(bytes);
    let streamed = compute_tarball_integrity_reader(std::io::Cursor::new(bytes)).unwrap();

    assert_eq!(streamed, buffered);
}
