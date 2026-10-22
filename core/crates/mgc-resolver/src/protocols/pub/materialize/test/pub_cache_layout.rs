//! Verify the Dart pub cache path and extracted package contents.
//! Kiểm tra đường dẫn cache Dart pub và nội dung package sau giải nén.

use super::super::PubProtocol;
use crate::protocols::ResolvedEntry;
use std::io::Cursor;

fn tar_gz_file(path: &str, contents: &[u8]) -> Vec<u8> {
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut archive = tar::Builder::new(encoder);
    let mut header = tar::Header::new_gnu();
    header.set_size(contents.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    archive
        .append_data(&mut header, path, Cursor::new(contents))
        .expect("test operation must succeed");
    archive
        .into_inner()
        .expect("test operation must succeed")
        .finish()
        .expect("test operation must succeed")
}

#[test]
fn materialize_uses_hosted_pub_dev_layout_and_extracts_contents() {
    let cache = tempfile::tempdir().expect("test operation must succeed");
    let archive = tar_gz_file("package/pubspec.yaml", b"name: sample\n");
    let entry = ResolvedEntry {
        name: "sample".to_string(),
        version: "1.2.3".to_string(),
        deps: Vec::new(),
        artifact_url: "https://pub.dev/packages/sample.tar.gz".to_string(),
        sha256: String::new(),
        extra_markers: Vec::new(),
    };

    let destination = PubProtocol::new("https://pub.dev")
        .materialize(&entry, &archive, cache.path())
        .expect("test operation must succeed");
    let expected = cache.path().join("hosted/pub.dev/sample-1.2.3");

    assert_eq!(destination, expected);
    assert_eq!(
        std::fs::read(destination.join("package/pubspec.yaml"))
            .expect("test operation must succeed"),
        b"name: sample\n"
    );
}
