//! Tests for streamed tarball digest output — test digest tarball dạng stream.

use super::digest_archive;
use base64::Engine;
use sha2::{Digest, Sha512};
use std::io::Write;

#[test]
fn hashes_small_archive_bytes_and_reports_compressed_size() {
    let mut staged = tempfile::NamedTempFile::new().expect("create archive fixture");
    staged
        .write_all(b"abc")
        .expect("write archive fixture bytes");

    let digest = digest_archive(staged.as_file_mut()).expect("digest archive fixture");

    assert_eq!(digest.shasum, "a9993e364706816aba3e25717850c26c9cd0d89d");
    assert_eq!(
        digest.integrity,
        format!(
            "sha512-{}",
            base64::engine::general_purpose::STANDARD.encode(Sha512::digest(b"abc"))
        )
    );
    assert_eq!(digest.size, 3);
}
