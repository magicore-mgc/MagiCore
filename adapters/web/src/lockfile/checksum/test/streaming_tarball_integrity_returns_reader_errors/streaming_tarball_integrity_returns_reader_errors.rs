// Preserve read errors and reject partial checksums — Giữ lỗi đọc và từ chối checksum thiếu.
#[test]
fn streaming_tarball_integrity_returns_reader_errors() {
    struct FailingReader;

    impl std::io::Read for FailingReader {
        fn read(&mut self, _buffer: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("fixture read failure"))
        }
    }

    let error = compute_tarball_integrity_reader(FailingReader).unwrap_err();

    assert_eq!(error.kind(), std::io::ErrorKind::Other);
}
