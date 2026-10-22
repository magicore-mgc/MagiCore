//! Select audit pins from the verified v4 owner or checked legacy Web lock.
//! Chọn pin audit từ owner v4 đã xác minh hoặc lock Web legacy đã kiểm tra.

use crate::audit::types::AuditPinSplit;
use crate::lockfile::read_web_lockfile_checked;
use std::path::Path;

use super::PinSourceError;

/// Return `None` only when no operational lockfile or rival lockfile exists.
/// Chỉ trả `None` khi không có lock vận hành lẫn lockfile đối thủ.
pub(in crate::audit) fn select_audit_pins(
    project_root: &Path,
) -> Result<Option<AuditPinSplit>, PinSourceError> {
    if let Some(v4_pins) = crate::audit::v4_pins_for_audit(project_root)? {
        return Ok(Some(v4_pins));
    }

    let Some(lockfile) = read_web_lockfile_checked(project_root)? else {
        let rival_lockfiles = detect_rival_lockfiles(project_root);
        if !rival_lockfiles.is_empty() {
            return Err(PinSourceError::RivalLockfiles(rival_lockfiles));
        }
        return Ok(None);
    };

    // Keep npm-auditable packages apart from JSR entries, which have no npm advisory lane.
    // Tách package npm khỏi JSR vì JSR chưa có lane advisory npm.
    let mut npm_pins: Vec<(String, String)> = lockfile
        .packages
        .iter()
        .filter(|package| !package.name.as_str().starts_with("jsr:"))
        .map(|package| (package.name.to_string(), package.version.clone()))
        .collect();
    let jsr_pins: Vec<String> = lockfile
        .packages
        .iter()
        .filter(|package| package.name.as_str().starts_with("jsr:"))
        .map(|package| package.name.as_str().to_string())
        .collect();

    npm_pins.sort();
    npm_pins.dedup();
    Ok(Some((npm_pins, jsr_pins)))
}

/// Check rival lockfile names only to provide explicit import remediation.
/// Chỉ dò tên lockfile đối thủ để đưa remediation import tường minh.
fn detect_rival_lockfiles(project_root: &Path) -> Vec<String> {
    ["bun.lock", "deno.lock", "bun.lockb"]
        .iter()
        .filter(|name| project_root.join(name).is_file())
        .map(|name| name.to_string())
        .collect()
}
