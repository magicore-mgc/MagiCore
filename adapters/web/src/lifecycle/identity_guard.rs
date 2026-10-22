//! Verify, restore, and fail closed when lifecycle hooks tamper with core identity.
//! Xác minh, phục hồi và chặn cài đặt khi hook sửa identity của core.

use super::error::{LifecycleError, LifecycleResult};
use mgc_config::project::{ProjectConfig, ProjectIdentitySnapshot};
use std::path::Path;

pub(super) fn ensure_project_identity(
    project_root: &Path,
    expected_owner: &Option<String>,
    identity_snapshot: &ProjectIdentitySnapshot,
) -> LifecycleResult<()> {
    let identity_matches =
        mgc_config::project::project_identity_matches_snapshot(project_root, identity_snapshot)
            .map_err(
                |source| match restore_identity(project_root, identity_snapshot) {
                    Ok(()) => LifecycleError::IdentityVerificationRestored { source },
                    Err(restore) => {
                        LifecycleError::IdentityVerificationRestoreFailed { source, restore }
                    }
                },
            )?;
    if !identity_matches {
        return Err(match restore_identity(project_root, identity_snapshot) {
            Ok(()) => LifecycleError::IdentityModifiedRestored,
            Err(restore) => LifecycleError::IdentityModifiedRestoreFailed { restore },
        });
    }

    let owner_after = ProjectConfig::detect_core(project_root).map_err(|source| {
        match restore_identity(project_root, identity_snapshot) {
            Ok(()) => LifecycleError::IdentityInvalidRestored { source },
            Err(restore) => LifecycleError::IdentityInvalidRestoreFailed { source, restore },
        }
    })?;
    if owner_after != *expected_owner {
        let expected = expected_owner.as_deref().unwrap_or("unclaimed").to_owned();
        let actual = owner_after.as_deref().unwrap_or("unclaimed").to_owned();
        return Err(match restore_identity(project_root, identity_snapshot) {
            Ok(()) => LifecycleError::IdentityChangedRestored { expected, actual },
            Err(restore) => LifecycleError::IdentityChangedRestoreFailed {
                expected,
                actual,
                restore,
            },
        });
    }
    Ok(())
}

fn restore_identity(
    project_root: &Path,
    identity_snapshot: &ProjectIdentitySnapshot,
) -> Result<(), anyhow::Error> {
    mgc_config::project::restore_project_identity(project_root, identity_snapshot)
}
