//! Project identity snapshots and restoration.
//! Chụp và khôi phục identity dự án.
use super::super::ProjectConfig;
use super::super::file_access::{
    atomic_write_project_config, ecosystem_from_config, ensure_regular_project_config,
    read_regular_project_text,
};
use std::path::Path;

/// Exact pre-operation snapshot of MagiCore-owned project identity files.
/// Snapshot nguyên byte các file định danh project do MGC sở hữu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectIdentitySnapshot {
    marker: Option<String>,
    config: Option<String>,
}

/// Capture the marker and project config before running untrusted lifecycle hooks.
/// Chụp marker và cấu hình trước khi chạy lifecycle hook không đáng tin.
pub fn snapshot_project_identity(project_root: &Path) -> anyhow::Result<ProjectIdentitySnapshot> {
    Ok(ProjectIdentitySnapshot {
        marker: read_regular_project_text(
            &project_root.join(ProjectConfig::CORE_MARKER_FILE),
            "core marker",
        )?,
        config: read_regular_project_text(&project_root.join("mgc.toml"), "project config")?,
    })
}

/// Check the marker bytes and canonical configured core against a snapshot.
/// So marker nguyên byte và core canonical trong config khớp snapshot.
pub fn project_identity_matches_snapshot(
    project_root: &Path,
    snapshot: &ProjectIdentitySnapshot,
) -> anyhow::Result<bool> {
    let marker = read_regular_project_text(
        &project_root.join(ProjectConfig::CORE_MARKER_FILE),
        "core marker",
    )?;
    if marker != snapshot.marker {
        return Ok(false);
    }
    let current_config =
        read_regular_project_text(&project_root.join("mgc.toml"), "project config")?;
    let current_core = ecosystem_from_config(current_config.as_deref())?;
    let expected_core = ecosystem_from_config(snapshot.config.as_deref())?;
    Ok(current_core.as_deref().map(ProjectConfig::canonical_core)
        == expected_core.as_deref().map(ProjectConfig::canonical_core))
}

/// Restore identity files atomically after a lifecycle hook changes ownership.
/// Refuses symlink/special-file targets and reports every failed restoration.
/// Khôi phục atomic file identity sau khi hook đổi ownership; từ chối symlink/file đặc biệt.
pub fn restore_project_identity(
    project_root: &Path,
    snapshot: &ProjectIdentitySnapshot,
) -> anyhow::Result<()> {
    let mut failures = Vec::new();
    for (path, content, label) in [
        (
            project_root.join("mgc.toml"),
            snapshot.config.as_deref(),
            "project config",
        ),
        (
            project_root.join(ProjectConfig::CORE_MARKER_FILE),
            snapshot.marker.as_deref(),
            "core marker",
        ),
    ] {
        let result = read_regular_project_text(&path, label).and_then(|current| {
            let unchanged = if label == "project config" {
                match (
                    ecosystem_from_config(current.as_deref()),
                    ecosystem_from_config(content),
                ) {
                    (Ok(current), Ok(expected)) => {
                        current.as_deref().map(ProjectConfig::canonical_core)
                            == expected.as_deref().map(ProjectConfig::canonical_core)
                    }
                    _ => false,
                }
            } else {
                current.as_deref() == content
            };
            if unchanged {
                return Ok(());
            }
            match content {
                Some(content) => atomic_write_project_config(&path, content.as_bytes()),
                None => remove_regular_project_identity_file(&path),
            }
        });
        if let Err(error) = result {
            failures.push(format!("restore {label} '{}': {error}", path.display()));
        }
    }
    if failures.is_empty() {
        Ok(())
    } else {
        anyhow::bail!(
            "project identity restoration failed: {}",
            failures.join("; ")
        )
    }
}

fn remove_regular_project_identity_file(path: &Path) -> anyhow::Result<()> {
    if ensure_regular_project_config(path)?.is_none() {
        return Ok(());
    }
    std::fs::remove_file(path)?;
    #[cfg(unix)]
    if let Some(parent) = path.parent() {
        std::fs::File::open(parent)?.sync_all()?;
    }
    Ok(())
}
