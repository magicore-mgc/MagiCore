//! Model file integrity verification and Git blob hashing.
//! Kiểm tra toàn vẹn file model và hash Git blob.

mod error;
mod file_integrity;

pub use file_integrity::git_blob_sha1;

#[cfg(test)]
mod test;
