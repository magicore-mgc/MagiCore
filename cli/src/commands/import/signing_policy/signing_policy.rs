//! Chooses signed output or the explicit unsigned escape hatch.
//! Chọn output có ký hoặc escape hatch unsigned được opt-in rõ.

use super::super::ImportError;

/// The only two output modes allowed by import policy.
/// Hai chế độ output duy nhất mà policy import cho phép.
#[derive(Debug, Eq, PartialEq)]
pub(in crate::commands::import) enum ImportSigningMode {
    Signed,
    UnsignedExplicit,
}

/// A default key always wins; unsigned output requires a separate explicit opt-in.
/// Có key mặc định luôn phải ký; unsigned cần opt-in tường minh riêng.
pub(in crate::commands::import) fn decide_signing_mode(
    has_default_key: bool,
    allow_unsigned: bool,
) -> Result<ImportSigningMode, ImportError> {
    if has_default_key {
        Ok(ImportSigningMode::Signed)
    } else if allow_unsigned {
        Ok(ImportSigningMode::UnsignedExplicit)
    } else {
        Err(ImportError::UnsignedOptInRequired)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
