//! Install one library version into the keg store and activate it.
//! Cài một version library vào store keg rồi kích hoạt.
//!
//! Installing a version that is already present replays activation against the
//! existing keg (idempotent) instead of failing, but content on disk is never
//! rewritten: a keg is immutable once sealed.
//! Cài version đã có sẽ kích hoạt lại trên keg cũ (idempotent), không ghi lại
//! đĩa: keg đã seal thì bất biến.
//!
//! If activation fails after a new keg was created, the new keg is removed so
//! the store does not hold a version the user cannot see as installed; the
//! previously active version was never touched.
//! Nếu kích hoạt lỗi sau khi đã tạo keg mới, keg mới bị xoá để store không giữ
//! version "không thấy được"; version active cũ không bị đụng.

use std::path::Path;

use crate::install::keg::error::KegResult;
use crate::install::keg::keg::{KegRecord, KegStore};
use crate::install::keg::switch::{SwitchOutcome, switch_active};

/// Result of installing and activating one version.
/// Kết quả cài + kích hoạt một version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallOutcome {
    /// Keg that now holds this version.
    /// Keg đang giữ version này.
    pub keg: KegRecord,
    /// True when the keg already existed (content was not rewritten).
    /// True khi keg đã tồn tại (không ghi lại nội dung).
    pub reused: bool,
    /// Activation result.
    /// Kết quả kích hoạt.
    pub activation: SwitchOutcome,
}

/// Install (`materialize`) one version and make it the active version.
/// Cài (`materialize`) một version và đưa thành version active.
pub fn install_and_activate<F>(
    store: &KegStore,
    name: &str,
    version: &str,
    materialize: F,
) -> KegResult<InstallOutcome>
where
    F: FnOnce(&Path) -> KegResult<()>,
{
    let existing = store.list_kegs(name)?;
    let already_present = existing.iter().any(|record| record.version == version);
    if already_present {
        // Verify instead of trusting the directory name.
        // (Xác minh thay vì tin tên thư mục.)
        let keg = store.open_keg(name, version)?;
        let activation = switch_active(store, name, version)?;
        return Ok(InstallOutcome {
            keg,
            reused: true,
            activation,
        });
    }

    let keg = store.create_keg(name, version, materialize)?;
    match switch_active(store, name, version) {
        Ok(activation) => Ok(InstallOutcome {
            keg,
            reused: false,
            activation,
        }),
        Err(error) => {
            // Roll the store back to its pre-install shape; the previous
            // active version is already restored by the failed switch.
            // (Đưa store về hình dạng trước khi cài; version active cũ đã được
            // switch lỗi khôi phục.)
            let _ = store.remove_keg(name, version);
            Err(error)
        }
    }
}
