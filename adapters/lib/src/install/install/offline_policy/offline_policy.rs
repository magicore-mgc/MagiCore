//! Enforce the existing Lib offline-install allowlist.
//! Áp dụng allowlist offline-install Lib hiện tại.

use crate::language::LibLanguage;
use mgc_types::{MgError, MgResult};

pub(in crate::install) fn validate_offline_install(
    language: LibLanguage,
    offline: bool,
) -> MgResult<()> {
    // Python and Rust support verified cache-only replay; other native
    // registry lanes still download during install and must refuse offline.
    // (Python và Rust hỗ trợ replay cache đã xác minh; lane native khác vẫn
    // tải từ registry nên phải từ chối offline.)
    if offline
        && !matches!(
            language,
            LibLanguage::Ts | LibLanguage::Python | LibLanguage::Rust
        )
    {
        return Err(MgError::Unsupported {
            core: "lib",
            capability: "offline install",
            guidance: "offline install is unsupported for this native ecosystem until cache-only frozen reinstall is implemented; no files were changed".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
