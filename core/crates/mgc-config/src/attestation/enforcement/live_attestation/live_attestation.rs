//! Live attestation policy / Policy chứng thực marker hiện hữu.
//! Checks the trust anchor after a project marker has passed validation.

use std::path::Path;

use crate::attestation::{AnchorMode, AttestationStatus, verify};

use super::super::warn_once;
use super::error::LiveAttestationError;

/// Enforce the live attestation for an already-validated core marker.
/// Called at the end of `read_core_marker` so every core dispatch is
/// covered. Absent anchor warns (backwards compatibility, unless
/// `require`); mismatching anchor fails in `require` mode and warns
/// otherwise — never silently re-anchored. Unreadable state warns and
/// proceeds in Warn mode; Require fails closed when home or state is
/// unavailable.
/// (Bắt buộc chứng thực cho marker đã validate. Vắng anchor thì cảnh
/// báo; lệch anchor thì fail ở mode require, cảnh báo ở mode warn —
/// không bao giờ tự ghi đè. Home/state không đọc được fail-closed ở
/// Require, còn Warn chỉ cảnh báo để giữ tương thích.)
pub fn enforce_live_attestation(project_root: &Path, core: &str) -> anyhow::Result<()> {
    let mode = AnchorMode::effective();
    if mode == AnchorMode::Off {
        return Ok(());
    }
    let identities = match mgc_platform::paths::GlobalPaths::new() {
        Ok(paths) => paths.identities,
        Err(source) => {
            let error = LiveAttestationError::GlobalIdentitiesUnavailable {
                source: source.into(),
            };
            if mode == AnchorMode::Require {
                return Err(error.into_anyhow());
            }
            warn_once(&format!("WARNING: {error}"));
            return Ok(());
        }
    };
    match verify(project_root, &identities) {
        Ok(AttestationStatus::Valid) => Ok(()),
        Ok(AttestationStatus::Absent) => {
            if mode == AnchorMode::Require {
                return Err(LiveAttestationError::Missing {
                    project: project_root.display().to_string(),
                    core: core.to_owned(),
                }
                .into_anyhow());
            }
            warn_once(&format!(
                concat!(
                    "WARNING: no core attestation for '{}' (core '{core}') — run `mgc trust anchor` to ",
                    "bind this project's core identity"
                ),
                project_root.display(),
                core = core
            ));
            Ok(())
        }
        // A signature from another core is a mismatch, never a warning-only
        // event: the distinction exists so the message names the real cause.
        // (Chữ ký từ core khác là mismatch, không bao giờ chỉ cảnh báo: phân biệt
        // để message nêu đúng nguyên nhân.)
        Ok(AttestationStatus::DomainMismatch(reason)) | Ok(AttestationStatus::Mismatch(reason)) => {
            if mode == AnchorMode::Require {
                return Err(LiveAttestationError::Mismatch {
                    project: project_root.display().to_string(),
                    core: core.to_owned(),
                    reason,
                }
                .into_anyhow());
            }
            warn_once(&format!(
                "WARNING: core attestation mismatch for '{core}' at '{}': {reason}",
                project_root.display()
            ));
            Ok(())
        }
        Err(source) => {
            if mode == AnchorMode::Require {
                return Err(LiveAttestationError::CannotVerify { source }.into_anyhow());
            }
            warn_once(&format!(
                "WARNING: cannot verify core attestation ({source}) — proceeding without anchor"
            ));
            Ok(())
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
