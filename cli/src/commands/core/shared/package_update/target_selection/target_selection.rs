//! Select all or explicitly named manifest dependencies for update.
//! Chọn dependency trong manifest theo danh sách hoặc toàn bộ để update.

use super::super::types::UpdateTarget;
use super::super::*;

/// Preserve package lookup order and fail on the first unknown name.
/// Giữ thứ tự tra package và dừng tại tên đầu tiên không tồn tại.
/// Installed `(name, version)` pairs from either lock schema.
///
/// `update` works on both v3 and v4 projects, so the reader hands over this flat
/// view instead of a schema-specific type. A `Vec` rather than a map because a
/// multi-version lock legitimately holds two entries for one name.
/// (`update` chạy trên cả project v3 và v4, nên reader trả view phẳng này thay vì
/// kiểu theo schema. Dùng `Vec` không phải map vì lock đa version hợp lệ có hai
/// entry cùng tên.)
pub(crate) type InstalledVersions = Vec<(String, String)>;

/// The version a dependency is ACTUALLY installed at.
///
/// Reading it from the manifest range reports the range's minimum, not the
/// installed version: a project declaring `nanoid: ^5.0.0` with `5.1.16` locked
/// was told "nanoid: 5.0.0 → 6.0.2", so the user believed the upgrade was a
/// bigger jump than it is and that the project ran 5.0.0. The lock is the
/// authority; the range is only a fallback for a dependency that was never
/// installed.
/// (Đọc từ range trong manifest báo MIN của range, không phải version đã cài.
/// Project khai `^5.0.0` với lock `5.1.16` từng được báo "5.0.0 → 6.0.2". Lock là
/// nguồn sự thật; range chỉ là fallback khi dependency chưa từng được cài.)
fn installed_version(
    dependency: &mgc_types::DependencySpec,
    lock: Option<&InstalledVersions>,
) -> String {
    if let Some(installed) = lock
        && let Some((_, version)) = installed
            .iter()
            .find(|(name, _)| name == dependency.name.as_str())
    {
        return version.clone();
    }
    dependency
        .range
        .satisfying_version()
        .map(|version| version.to_string())
        .unwrap_or_else(|| dependency.range.to_string())
}

pub(in crate::commands::core::shared) fn select_update_targets(
    packages: &[String],
    manifest: &Manifest,
    lock: Option<&InstalledVersions>,
) -> Result<Vec<UpdateTarget>> {
    if packages.is_empty() {
        Ok(manifest
            .all_dependencies()
            .map(|dependency| {
                let current_version = installed_version(dependency, lock);
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
            let current_version = installed_version(dependency, lock);
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

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
