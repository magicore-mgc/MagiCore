//! Safe tarball reading and content-signature construction.
//! Đọc tarball an toàn và dựng chữ ký nội dung.

mod archive_path;
mod archive_signature;
mod entry_scan;
mod tree_signature;

pub use archive_path::{
    common_tarball_root_prefix, path_to_signature_string, sanitize_tarball_signature_path,
};
pub use archive_signature::{
    tarball_content_signature, tarball_content_signature_from_path,
    tarball_content_signature_from_reader,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
