//! Project-core claim and atomic marker publication.
//! Claim core project và publish marker nguyên tử.
use std::path::Path;

use super::super::super::ProjectConfig;
use super::super::super::file_access::ensure_regular_project_config;
use super::super::error::{CoreMarkerError, Result};
use super::super::{identity, reader};

fn ensure_mgc_config_core_compatible(project_root: &Path, desired_core: &str) -> Result<()> {
    let path = project_root.join("mgc.toml");
    if ensure_regular_project_config(&path)?.is_none() {
        return Ok(());
    }
    let Some(existing) = ProjectConfig::load(project_root)? else {
        return Ok(());
    };
    let existing_core = identity::canonical_core(&existing.ecosystem);
    if existing_core != desired_core {
        return Err(CoreMarkerError::ConfiguredCoreReassignment {
            existing: existing_core,
            desired: desired_core.to_string(),
        });
    }
    Ok(())
}

/// Write the marker for a project configuration's core.
/// Ghi marker theo core của project config.
pub(in crate::project::project::core_marker) fn write_core_marker(
    project_root: &Path,
    core: &str,
) -> Result<()> {
    ensure_core_marker_at(project_root, core)
}

/// Claim a missing marker or verify that an existing marker has the same core.
/// Claim marker còn thiếu hoặc xác minh marker hiện tại cùng core.
pub(in crate::project::project::core_marker) fn ensure_core_marker_at(
    project_root: &Path,
    core: &str,
) -> Result<()> {
    let canonical = identity::canonical_core(core);
    if !identity::is_known_core(&canonical) {
        return Err(CoreMarkerError::UnknownCore {
            core: core.to_string(),
            known: ProjectConfig::KNOWN_CORES.join(", "),
        });
    }
    // Keep this compatibility check in the shared ownership primitive so each
    // public entry point is unable to override an existing mgc.toml identity.
    // (Đặt guard ở primitive dùng chung để mọi API đều chặn ghi đè identity.)
    ensure_mgc_config_core_compatible(project_root, &canonical)?;
    std::fs::create_dir_all(project_root)?;
    let path = project_root.join(ProjectConfig::CORE_MARKER_FILE);
    match reader::read_core_marker(project_root)? {
        Some(existing) if existing == canonical => return Ok(()),
        Some(existing) => {
            return Err(CoreMarkerError::ExistingMarkerConflict {
                existing,
                desired: canonical,
            });
        }
        None => {}
    }

    // Publish fully written bytes with a no-replace hard link in the same directory.
    // Ghi xong toàn bộ rồi hard-link không ghi đè trong cùng thư mục.
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let temp_path = parent.join(format!(
        ".{}.claim.{}.{}",
        ProjectConfig::CORE_MARKER_FILE,
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    let result = (|| -> Result<()> {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
        }
        let mut file = options.open(&temp_path)?;
        use std::io::Write;
        file.write_all(format!("{canonical}\n").as_bytes())?;
        file.sync_all()?;
        match std::fs::hard_link(&temp_path, &path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                return match reader::read_core_marker(project_root)? {
                    Some(existing) if existing == canonical => Ok(()),
                    Some(existing) => Err(CoreMarkerError::ConcurrentMarkerConflict {
                        existing,
                        desired: canonical.clone(),
                    }),
                    None => Err(CoreMarkerError::MarkerChangedDuringClaim { path: path.clone() }),
                };
            }
            Err(error) => return Err(error.into()),
        }
        #[cfg(unix)]
        std::fs::File::open(parent)?.sync_all()?;
        Ok(())
    })();
    let cleanup = std::fs::remove_file(&temp_path);
    if let Err(error) = result {
        let _ = cleanup;
        return Err(error);
    }
    cleanup?;
    Ok(())
}

/// Write only after checking that persisted project configuration agrees.
/// Chỉ ghi sau khi kiểm tra config đã lưu có cùng core.
pub(in crate::project::project::core_marker) fn write_core_marker_at(
    project_root: &Path,
    core: &str,
) -> Result<()> {
    let canonical = identity::canonical_core(core);
    if !identity::is_known_core(&canonical) {
        return Err(CoreMarkerError::UnknownCore {
            core: core.to_string(),
            known: ProjectConfig::KNOWN_CORES.join(", "),
        });
    }
    std::fs::create_dir_all(project_root)?;
    ensure_core_marker_at(project_root, &canonical)
}
