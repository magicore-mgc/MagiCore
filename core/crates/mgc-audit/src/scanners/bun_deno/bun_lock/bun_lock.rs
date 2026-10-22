//! Parse Bun lockfile v1 entries — phân tích entry Bun lockfile v1.
//! JSONC normalization remains delegated to the shared `mgc-types` owner.
//! Chuẩn hóa JSONC tiếp tục do owner dùng chung `mgc-types` đảm nhiệm.

use crate::scanners::bun_deno::{BunDenoRead, JrSkipped, NpmPin};
use mgc_types::{MgError, MgResult, strip_jsonc};
use serde::Deserialize;

use super::error::BunLockError;

#[derive(Debug, Deserialize)]
struct BunLock {
    #[serde(default)]
    packages: serde_json::Map<String, serde_json::Value>,
}

/// Read `bun.lock` (JSONC, trailing commas) into npm pins. Only entries
/// whose first array element parses as `name@version` are npm deps;
/// workspace/registry entries without that shape are skipped with a reason.
/// Đọc `bun.lock` (JSONC, dấu phẩy cuối) thành ghim npm. Chỉ entry có
/// phần tử mảng đầu parse được `name@version` là dep npm; entry khác dạng
/// bị bỏ qua kèm lý do.
pub fn read_bun_lock(raw: &str) -> MgResult<BunDenoRead> {
    let cleaned = strip_jsonc(raw);
    let lock: BunLock = serde_json::from_str(&cleaned)
        .map_err(BunLockError::InvalidJson)
        .map_err(MgError::from)?;

    let mut read = BunDenoRead::default();
    for (key, entry) in &lock.packages {
        let Some(arr) = entry.as_array() else {
            return Err(BunLockError::PackageEntryNotArray(key.clone()).into());
        };
        let Some(first) = arr.first().and_then(|value| value.as_str()) else {
            return Err(BunLockError::FirstEntryNotString(key.clone()).into());
        };
        match super::super::split_name_version(first) {
            Some((name, version)) => read.npm_pins.push(NpmPin { name, version }),
            None => read.jsr_skipped.push(JrSkipped {
                specifier: first.to_string(),
            }),
        }
    }
    Ok(read)
}
