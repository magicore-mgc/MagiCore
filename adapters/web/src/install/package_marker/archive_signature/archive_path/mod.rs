//! Tar entry path sanitation and common-root normalization.
//! Làm sạch path tar và chuẩn hóa root chung.

mod archive_path;

pub use archive_path::{
    common_tarball_root_prefix, path_to_signature_string, sanitize_tarball_signature_path,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
