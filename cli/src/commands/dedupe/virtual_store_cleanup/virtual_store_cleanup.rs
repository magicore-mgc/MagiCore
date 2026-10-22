//! Virtual-store cleanup — validates roots and counts only completed logical removals.
//! Dọn virtual store — kiểm tra root và chỉ cộng payload logic đã xóa xong.
use std::fs;
use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;

use super::error::VirtualStoreCleanupError;
use super::measurement::logical_payload_bytes;
use super::root::existing_store_root;

#[cfg(test)]
pub(in crate::commands::dedupe) fn vstore_root(project_root: &Path) -> PathBuf {
    project_root.join("node_modules").join(".magicore")
}

/// Delete only unreferenced real package directories from a validated store root.
/// Chỉ xóa package directory thật, không còn được tham chiếu trong root đã kiểm tra.
pub(in crate::commands::dedupe) fn cleanup_unreferenced_vstore(
    project_root: &Path,
    lock: &mgc_lockfile::Lockfile,
) -> GcReport {
    let mut report = GcReport::default();
    let store_root = match existing_store_root(project_root) {
        Ok(Some(path)) => path,
        Ok(None) => return report,
        Err(error) => {
            report.failed.push(error);
            return report;
        }
    };

    let entries = match fs::read_dir(&store_root) {
        Ok(entries) => entries,
        Err(source) => {
            report.failed.push(VirtualStoreCleanupError::ReadDirectory {
                path: display_path(&store_root),
                source,
            });
            return report;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(source) => {
                report.failed.push(VirtualStoreCleanupError::ReadEntry {
                    path: display_path(&store_root),
                    source,
                });
                continue;
            }
        };
        let package_path = entry.path();
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(source) => {
                report.failed.push(VirtualStoreCleanupError::InspectEntry {
                    path: display_path(&package_path),
                    source,
                });
                continue;
            }
        };
        if file_type.is_symlink() {
            report
                .failed
                .push(VirtualStoreCleanupError::SymlinkedPackageEntry {
                    path: display_path(&package_path),
                });
            continue;
        }
        if !file_type.is_dir() {
            continue;
        }

        let package_name = match package_path.file_name().and_then(|name| name.to_str()) {
            Some(name) => name.to_owned(),
            None => {
                report
                    .failed
                    .push(VirtualStoreCleanupError::InvalidPackageName {
                        path: display_path(&package_path),
                    });
                continue;
            }
        };
        let referenced = lock.packages.iter().any(|package| {
            format!("{}@{}", package.name.replace('/', "+"), package.version) == package_name
        });
        if referenced {
            continue;
        }

        let canonical_package = match fs::canonicalize(&package_path) {
            Ok(path) => path,
            Err(source) => {
                report
                    .failed
                    .push(VirtualStoreCleanupError::CanonicalizeDirectory {
                        path: display_path(&package_path),
                        source,
                    });
                continue;
            }
        };
        if !canonical_package.starts_with(&store_root) {
            report
                .failed
                .push(VirtualStoreCleanupError::PathEscapesBoundary {
                    path: display_path(&canonical_package),
                    boundary: display_path(&store_root),
                });
            continue;
        }
        let metadata = match fs::symlink_metadata(&package_path) {
            Ok(metadata) => metadata,
            Err(source) => {
                report.failed.push(VirtualStoreCleanupError::InspectEntry {
                    path: display_path(&package_path),
                    source,
                });
                continue;
            }
        };
        if metadata.file_type().is_symlink() {
            report
                .failed
                .push(VirtualStoreCleanupError::SymlinkedPackageEntry {
                    path: display_path(&package_path),
                });
            continue;
        }
        if !metadata.is_dir() {
            report
                .failed
                .push(VirtualStoreCleanupError::EntryChangedType {
                    path: display_path(&package_path),
                });
            continue;
        }

        let bytes = match logical_payload_bytes(&canonical_package) {
            Ok(bytes) => bytes,
            Err(error) => {
                report.failed.push(error);
                continue;
            }
        };
        let Some(total_after_delete) = report
            .logical_payload_bytes_in_removed_paths
            .checked_add(bytes)
        else {
            report
                .failed
                .push(VirtualStoreCleanupError::ReportSizeOverflow {
                    path: display_path(&canonical_package),
                });
            continue;
        };

        match fs::remove_dir_all(&canonical_package) {
            Ok(()) => {
                report.deleted += 1;
                report.logical_payload_bytes_in_removed_paths = total_after_delete;
            }
            Err(source) => {
                report
                    .failed
                    .push(VirtualStoreCleanupError::RemoveDirectory {
                        path: display_path(&canonical_package),
                        source,
                    });
            }
        }
    }
    report
}

/// Cleanup failures remain visible; byte totals describe logical payload, not disk blocks.
/// Lỗi cleanup vẫn được báo; tổng byte là payload logic, không phải block SSD đã thu hồi.
#[derive(Debug, Default)]
pub(in crate::commands::dedupe) struct GcReport {
    deleted: usize,
    logical_payload_bytes_in_removed_paths: u64,
    failed: Vec<VirtualStoreCleanupError>,
}

impl GcReport {
    #[cfg(test)]
    pub(in crate::commands::dedupe) fn deleted_count(&self) -> usize {
        self.deleted
    }

    pub(in crate::commands::dedupe) fn logical_payload_bytes_in_removed_paths(&self) -> u64 {
        self.logical_payload_bytes_in_removed_paths
    }

    pub(in crate::commands::dedupe) fn failed_count(&self) -> usize {
        self.failed.len()
    }

    pub(in crate::commands::dedupe) fn failure_messages(&self) -> Vec<String> {
        self.failed.iter().map(ToString::to_string).collect()
    }
}

fn display_path(path: &Path) -> String {
    path.display().to_string()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
