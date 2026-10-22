//! Wires signature-sidecar path and presence operations.
//! Nối thao tác xác định path và kiểm tra sidecar chữ ký.

mod signature;

#[allow(deprecated)]
pub use self::signature::is_lockfile_signed;
pub use self::signature::{signature_file_presence, signature_path_for};
