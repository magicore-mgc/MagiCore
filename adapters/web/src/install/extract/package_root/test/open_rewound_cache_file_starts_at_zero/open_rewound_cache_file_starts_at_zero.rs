// Keep extraction on the same opened cache file — Giữ extraction trên cùng file cache đã mở.
#[test]
fn open_rewound_cache_file_starts_at_zero() {
    use std::io::{Read, Seek, SeekFrom};
    use std::sync::{Arc, Mutex};

    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("verified.tgz");
    std::fs::write(&path, b"verified file contents").unwrap();
    let mut source = std::fs::File::open(path).unwrap();
    source.seek(SeekFrom::End(0)).unwrap();
    let source = Arc::new(Mutex::new(source));

    let mut reader = open_rewound_cache_file(&source).unwrap();
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes).unwrap();

    assert_eq!(bytes, b"verified file contents");
}
