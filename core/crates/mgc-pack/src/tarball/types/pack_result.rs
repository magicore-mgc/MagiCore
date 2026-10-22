//! Tarball result data contract — hợp đồng dữ liệu kết quả tarball.

use std::path::PathBuf;

/// Archive path, digests, compressed/unpacked sizes and entry count.
/// Path archive, digest, kích thước nén/giải nén và số entry.
pub struct PackResult {
    pub tarball_path: PathBuf,
    pub shasum: String,
    pub integrity: String,
    pub size: u64,
    pub unpacked_size: u64,
    pub entry_count: usize,
}
