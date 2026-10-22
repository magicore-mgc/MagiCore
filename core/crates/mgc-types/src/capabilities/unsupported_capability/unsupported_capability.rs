//! Unsupported error constructor implementation.
//! Triển khai hàm tạo lỗi unsupported.

use crate::error::MgError;

/// Fail-closed `Unsupported` error in the established message style
/// (`{core} core does not support '{capability}' yet: {guidance}`).
/// Lỗi `Unsupported` fail-closed đúng style message hiện hành.
pub fn unsupported_capability(
    core: &'static str,
    capability: &'static str,
    guidance: &str,
) -> MgError {
    MgError::Unsupported {
        core,
        capability,
        guidance: guidance.to_string(),
    }
}
