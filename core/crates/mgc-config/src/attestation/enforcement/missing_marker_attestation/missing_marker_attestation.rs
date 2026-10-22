//! Missing-marker policy / Policy marker bị thiếu.
//! Prevents fallback when a trusted core marker disappears.

use std::path::Path;

use crate::attestation::{AnchorMode, AttestationStatus, verify};

use super::super::warn_once;
use super::error::MissingMarkerAttestationError;

/// Check trust state when `.mgc.core` is missing, including when it was renamed.
/// Kiểm tra trust khi thiếu `.mgc.core`, kể cả trường hợp file bị đổi tên.
///
/// An unanchored legacy project remains discoverable in the default `warn`
/// mode. `require` rejects missing identity and rejects an existing attestation
/// whose marker disappeared, so detection cannot silently fall back to another
/// manifest after a trusted marker is removed.
/// (Project cũ chưa anchor vẫn được nhận diện ở chế độ `warn`; `require` chặn
/// identity bị thiếu hoặc marker đã ký bị xóa/đổi tên.)
pub fn enforce_missing_marker_attestation(project_root: &Path) -> anyhow::Result<()> {
    let mode = AnchorMode::effective();
    if mode == AnchorMode::Off {
        return Ok(());
    }
    let identities = match mgc_platform::paths::GlobalPaths::new() {
        Ok(paths) => paths.identities,
        Err(source) => {
            let error = MissingMarkerAttestationError::GlobalIdentitiesUnavailable {
                source: source.into(),
            };
            if mode == AnchorMode::Require {
                return Err(error.into_anyhow());
            }
            return Ok(());
        }
    };

    match verify(project_root, &identities) {
        Ok(AttestationStatus::Absent) if mode == AnchorMode::Require => {
            Err(MissingMarkerAttestationError::MissingIdentity {
                project: project_root.display().to_string(),
                marker: crate::project::ProjectConfig::CORE_MARKER_FILE.to_owned(),
            }
            .into_anyhow())
        }
        Ok(AttestationStatus::Absent | AttestationStatus::Valid) => Ok(()),
        Ok(AttestationStatus::Mismatch(reason)) if mode == AnchorMode::Require => {
            Err(MissingMarkerAttestationError::Mismatch {
                project: project_root.display().to_string(),
                reason,
            }
            .into_anyhow())
        }
        Ok(AttestationStatus::Mismatch(reason)) => {
            warn_once(&format!(
                "WARNING: core identity attestation mismatch at '{}': {reason}",
                project_root.display(),
            ));
            Ok(())
        }
        Err(source) if mode == AnchorMode::Require => {
            Err(MissingMarkerAttestationError::CannotVerify { source }.into_anyhow())
        }
        Err(error) => {
            warn_once(&format!(
                "WARNING: cannot verify core identity attestation ({error}) — proceeding without anchor"
            ));
            Ok(())
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
