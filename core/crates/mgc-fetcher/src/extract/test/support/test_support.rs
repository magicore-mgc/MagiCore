//! Builds compact gzip-tar fixtures for extraction unit tests only.
//! Tạo fixture gzip-tar gọn chỉ dùng cho unit test giải nén.

use flate2::Compression;
use flate2::write::GzEncoder;
use std::io::Cursor;
use tar::{Builder, EntryType, Header};

pub(in crate::extract) fn gzip_tar_file(path: &str, contents: &[u8]) -> Vec<u8> {
    gzip_tar_file_with_mode(path, contents, 0o644)
}

pub(in crate::extract) fn gzip_tar_file_with_mode(
    path: &str,
    contents: &[u8],
    mode: u32,
) -> Vec<u8> {
    gzip_tar_entries(&[(path, contents, mode)])
}

pub(in crate::extract) fn gzip_tar_files(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let entries_with_modes: Vec<_> = entries
        .iter()
        .map(|(path, contents)| (*path, *contents, 0o644))
        .collect();
    gzip_tar_entries(&entries_with_modes)
}

pub(in crate::extract) fn gzip_tar_link(path: &str, target: &str, kind: EntryType) -> Vec<u8> {
    let encoder = GzEncoder::new(Vec::new(), Compression::default());
    let mut archive = Builder::new(encoder);
    let mut header = Header::new_gnu();
    header.set_entry_type(kind);
    header.set_size(0);
    header.set_link_name(target).unwrap();
    header.set_cksum();
    archive
        .append_data(&mut header, path, std::io::empty())
        .unwrap();
    archive.into_inner().unwrap().finish().unwrap()
}

fn gzip_tar_entries(entries: &[(&str, &[u8], u32)]) -> Vec<u8> {
    let encoder = GzEncoder::new(Vec::new(), Compression::default());
    let mut archive = Builder::new(encoder);
    for (path, contents, mode) in entries {
        let mut header = Header::new_gnu();
        header.set_size(contents.len() as u64);
        header.set_mode(*mode);
        header.set_cksum();
        archive
            .append_data(&mut header, *path, Cursor::new(*contents))
            .unwrap();
    }
    archive.into_inner().unwrap().finish().unwrap()
}
