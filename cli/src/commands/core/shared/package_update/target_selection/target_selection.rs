//! Select all or explicitly named manifest dependencies for update.
//! Chọn dependency trong manifest theo danh sách hoặc toàn bộ để update.

use super::super::types::UpdateTarget;
use super::super::*;

/// Preserve package lookup order and fail on the first unknown name.
/// Giữ thứ tự tra package và dừng tại tên đầu tiên không tồn tại.
pub(crate) fn select_update_targets(
    packages: &[String],
    manifest: &Manifest,
) -> Result<Vec<UpdateTarget>> {
    if packages.is_empty() {
        Ok(manifest
            .all_dependencies()
            .map(|dependency| {
                let current_version = dependency
                    .range
                    .satisfying_version()
                    .map(|version| version.to_string())
                    .unwrap_or_else(|| dependency.range.to_string());
                UpdateTarget {
                    name: dependency.name.as_str().to_string(),
                    current_version,
                    dev: dependency.dev,
                    optional: dependency.optional,
                    peer: dependency.peer,
                }
            })
            .collect())
    } else {
        let mut targets = Vec::new();
        for name in packages {
            let Some(dependency) = manifest.find_dep(name) else {
                return Err(crate::error::update_unknown_package(name));
            };
            let current_version = dependency
                .range
                .satisfying_version()
                .map(|version| version.to_string())
                .unwrap_or_else(|| dependency.range.to_string());
            targets.push(UpdateTarget {
                name: dependency.name.as_str().to_string(),
                current_version,
                dev: dependency.dev,
                optional: dependency.optional,
                peer: dependency.peer,
            });
        }
        Ok(targets)
    }
}
