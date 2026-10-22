//! Parse Deno lockfile npm and JSR pins — phân tích pin npm và JSR trong lockfile Deno.
//! JSR-only dependencies remain visible as skipped rather than silently omitted.
//! Dependency chỉ có JSR vẫn hiện diện trong danh sách skipped, không bị bỏ âm thầm.

use crate::scanners::bun_deno::{BunDenoRead, JrSkipped, NpmPin};
use mgc_types::{MgError, MgResult};
use serde::Deserialize;

use super::error::DenoLockError;

#[derive(Debug, Deserialize)]
struct DenoLock {
    #[serde(default)]
    npm: serde_json::Map<String, serde_json::Value>,
    #[serde(default)]
    jsr: serde_json::Map<String, serde_json::Value>,
}

/// Read `deno.lock` (v5) into npm pins and explicitly skipped JSR deps.
/// Đọc `deno.lock` (v5) thành ghim npm và dependency JSR được ghi rõ là bỏ qua.
pub fn read_deno_lock(raw: &str) -> MgResult<BunDenoRead> {
    let lock: DenoLock = serde_json::from_str(raw)
        .map_err(DenoLockError::InvalidJson)
        .map_err(MgError::from)?;

    let mut read = BunDenoRead::default();
    for pin in lock.npm.keys() {
        let Some((name, version)) = super::super::split_name_version(pin) else {
            return Err(DenoLockError::NpmPinWithoutVersion(pin.clone()).into());
        };
        read.npm_pins.push(NpmPin { name, version });
    }
    for pin in lock.jsr.keys() {
        read.jsr_skipped.push(JrSkipped {
            specifier: pin.to_string(),
        });
    }
    Ok(read)
}
