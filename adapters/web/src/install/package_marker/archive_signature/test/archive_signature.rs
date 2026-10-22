//! Public tarball signature contracts.
//! Contract chữ ký tarball public.

use flate2::Compression;
use flate2::write::GzEncoder;
use tar::{Builder, Header};

use super::super::tarball_content_signature;

fn tarball_with_package_file() -> Vec<u8> {
    let encoder = GzEncoder::new(Vec::new(), Compression::default());
    let mut archive = Builder::new(encoder);
    let mut header = Header::new_gnu();
    header.set_size(3);
    header.set_mode(0o644);
    header.set_cksum();
    archive
        .append_data(&mut header, "package/index.js", &b"abc"[..])
        .unwrap();
    archive.into_inner().unwrap().finish().unwrap()
}

#[test]
fn tarball_signature_counts_unpacked_files_and_bytes() {
    let signature = tarball_content_signature(&tarball_with_package_file()).unwrap();

    assert_eq!(signature.file_count, 1);
    assert_eq!(signature.unpacked_size, 3);
    assert!(!signature.file_tree_sha256.is_empty());
}
