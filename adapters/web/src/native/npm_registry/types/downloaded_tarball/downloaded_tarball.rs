//! types downloaded tarball.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

pub enum DownloadedTarball {
    Bytes(Vec<u8>),
    Streamed {
        computed_integrity: String,
        bytes_len: u64,
    },
}
