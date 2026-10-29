//! In-memory tar.gz fixtures shared by archive owner tests.
//! Fixture tar.gz trong bộ nhớ dùng chung cho test các owner archive.

use std::io::Cursor;

pub(in crate::protocols::archive) fn tar_gz_file(path: &str, bytes: &[u8]) -> Vec<u8> {
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut archive = tar::Builder::new(encoder);
    let mut header = tar::Header::new_gnu();
    header.set_size(bytes.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    archive
        .append_data(&mut header, path, Cursor::new(bytes))
        .expect("test operation must succeed");
    archive
        .into_inner()
        .expect("test operation must succeed")
        .finish()
        .expect("test operation must succeed")
}

/// Build a hostile tar header without the safe `append_data` path validator.
/// Tạo tar header đối kháng mà không qua kiểm tra path an toàn của `append_data`.
pub(in crate::protocols::archive) fn tar_gz_file_with_raw_path(
    path: &str,
    bytes: &[u8],
) -> Vec<u8> {
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut archive = tar::Builder::new(encoder);
    let path_bytes = path.as_bytes();
    assert!(
        path_bytes.len() <= 100,
        "fixture path must fit in the tar name field"
    );
    let mut header = tar::Header::new_gnu();
    header.as_mut_bytes()[..100].fill(0);
    header.as_mut_bytes()[..path_bytes.len()].copy_from_slice(path_bytes);
    header.set_size(bytes.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    archive
        .append(&header, Cursor::new(bytes))
        .expect("test operation must succeed");
    archive
        .into_inner()
        .expect("test operation must succeed")
        .finish()
        .expect("test operation must succeed")
}

pub(crate) fn tar_gz_entries(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut archive = tar::Builder::new(encoder);
    for (path, bytes) in entries {
        let mut header = tar::Header::new_gnu();
        header.set_size(bytes.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        archive
            .append_data(&mut header, *path, Cursor::new(*bytes))
            .expect("test operation must succeed");
    }
    archive
        .into_inner()
        .expect("test operation must succeed")
        .finish()
        .expect("test operation must succeed")
}
