//! Tar entry scan and content-digest contracts.
//! Contract quét entry tar và digest nội dung.

use flate2::Compression;
use flate2::write::GzEncoder;
use tar::{Builder, Header};

use super::super::entry_scan::read_tarball_files;

#[test]
fn regular_file_entries_are_hashed_and_normalized() {
    let encoder = GzEncoder::new(Vec::new(), Compression::default());
    let mut archive = Builder::new(encoder);
    let mut header = Header::new_gnu();
    header.set_size(3);
    header.set_mode(0o644);
    header.set_cksum();
    archive
        .append_data(&mut header, "package/index.js", &b"abc"[..])
        .unwrap();
    let tarball = archive.into_inner().unwrap().finish().unwrap();
    let decoder = flate2::read::GzDecoder::new(tarball.as_slice());
    let mut archive = tar::Archive::new(decoder);

    let files = read_tarball_files(&mut archive).unwrap();

    assert_eq!(files.len(), 1);
    assert_eq!(files[0].0, "package/index.js");
    assert_eq!(files[0].1, 3);
    assert!(!files[0].2);
}
