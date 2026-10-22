//! Validate a SHA-256 digest before checksum read/write.
//! Kiểm tra định dạng digest SHA-256 trước khi đọc/ghi checksum.

use mgc_types::{MgError, MgResult};

pub(in crate::install::cargo_compat) fn validate_sha256(value: &str) -> MgResult<()> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(MgError::Other(
            "Cargo projection requires a 64-digit SHA-256 package checksum".to_string(),
        ));
    }
    Ok(())
}
