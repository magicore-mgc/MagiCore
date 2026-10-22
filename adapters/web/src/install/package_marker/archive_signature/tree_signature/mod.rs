//! Normalize archive file paths and compute the final tree signature.
//! Chuẩn hóa path file archive và tính chữ ký cây cuối cùng.

mod tree_signature;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

pub(in crate::install::package_marker::archive_signature) use tree_signature::signature_from_files;
