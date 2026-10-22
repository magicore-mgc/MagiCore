//! Validate supported App install lanes and offline eligibility.
//! Kiểm tra lane install App được hỗ trợ và điều kiện offline.

use crate::language::AppLanguage;
use mgc_types::{MgError, MgResult};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(in crate::install::install) fn ensure_dispatchable_language(
    language: AppLanguage,
) -> MgResult<()> {
    if !matches!(
        language,
        AppLanguage::Flutter | AppLanguage::Swift | AppLanguage::ReactNative
    ) {
        return Err(MgError::Unsupported {
            core: "app",
            capability: "install",
            guidance: "MagiCore has no complete native dependency engine for this app ecosystem"
                .to_string(),
        });
    }

    Ok(())
}

pub(in crate::install::install) fn ensure_offline_supported(
    language: AppLanguage,
    offline: bool,
) -> MgResult<()> {
    if offline && language != AppLanguage::Flutter {
        return Err(MgError::Unsupported {
            core: "app",
            capability: "offline install",
            guidance: "offline install is unsupported for this app ecosystem until cache-only frozen reinstall is implemented; no files were changed".to_string(),
        });
    }

    Ok(())
}
