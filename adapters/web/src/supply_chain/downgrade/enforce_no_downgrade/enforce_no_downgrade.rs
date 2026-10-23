//! Compare new direct top-level selections with the prior valid lock.
//! So sánh lựa chọn top-level dependency trực tiếp mới với lock hợp lệ trước đó.
use std::collections::HashMap;
use std::path::Path;

use mgc_resolver::solver::RootResolution;
use mgc_types::{Manifest, adapter::ResolvedGraph};

use super::super::super::error::SupplyChainError;

#[path = "pins/mod.rs"]
mod pins;

use pins::read_previous_direct_pins;

/// Reject direct-dependency regressions against a lock that satisfies the manifest.
/// Từ chối downgrade dependency trực tiếp so với lock còn thỏa manifest.
pub(in crate::supply_chain) fn enforce_no_downgrade(
    root_resolutions: &[RootResolution],
    manifest: &Manifest,
) -> Result<(), SupplyChainError> {
    let cwd = std::env::current_dir().map_err(|source| SupplyChainError::ProjectRoot { source })?;
    enforce_no_downgrade_at(root_resolutions, manifest, &cwd)
}

fn enforce_no_downgrade_at(
    root_resolutions: &[RootResolution],
    manifest: &Manifest,
    project_root: &Path,
) -> Result<(), SupplyChainError> {
    // Resolver provenance must form a one-to-one ordered projection of the manifest.
    // Provenance resolver phải ánh xạ một-một theo thứ tự manifest.
    let mut roots_by_index = HashMap::with_capacity(root_resolutions.len());
    for root in root_resolutions {
        if roots_by_index.insert(root.request_index, root).is_some() {
            return Err(SupplyChainError::DuplicateRootResolution {
                index: root.request_index,
            });
        }
    }

    let direct_dependency_count = manifest.all_dependencies().count();
    if roots_by_index.len() != direct_dependency_count {
        return Err(SupplyChainError::RootResolutionCount {
            expected: direct_dependency_count,
            actual: roots_by_index.len(),
        });
    }

    for (request_index, dep) in manifest.all_dependencies().enumerate() {
        let root = roots_by_index.get(&request_index).ok_or_else(|| {
            SupplyChainError::MissingRootResolution {
                index: request_index,
                name: dep.name.as_str().to_owned(),
            }
        })?;
        if root.request_name.as_str() != dep.name.as_str()
            || root.package_id.name_str() != dep.name.as_str()
        {
            return Err(SupplyChainError::InvalidRootResolution {
                index: request_index,
                expected_name: dep.name.as_str().to_owned(),
                actual_request_name: root.request_name.as_str().to_owned(),
                actual_package_name: root.package_id.name_str().to_owned(),
            });
        }
    }

    let previous_top = read_previous_direct_pins(project_root, manifest)?;
    if previous_top.is_empty() {
        return Ok(());
    }

    for (request_index, dep) in manifest.all_dependencies().enumerate() {
        let name = dep.name.as_str();
        let Some(baseline) = previous_top.get(&request_index) else {
            continue;
        };
        let root = roots_by_index.get(&request_index).ok_or_else(|| {
            SupplyChainError::MissingRootResolution {
                index: request_index,
                name: name.to_owned(),
            }
        })?;
        let new_version = root.package_id.version();
        if new_version < baseline {
            return Err(SupplyChainError::Downgrade {
                name: name.to_owned(),
                previous: Box::new(baseline.clone()),
                selected: Box::new(new_version.clone()),
            });
        }
    }
    Ok(())
}

/// Reconstruct indexed root selections from a normalized shared-cache graph.
/// Dựng lại lựa chọn root có index từ graph shared-cache đã chuẩn hóa.
pub(in crate::supply_chain) fn cached_root_resolutions(
    graph: &ResolvedGraph,
    manifest: &Manifest,
) -> Result<Vec<RootResolution>, SupplyChainError> {
    let mut roots = Vec::new();
    for (request_index, dependency) in manifest.all_dependencies().enumerate() {
        let mut candidates = graph
            .packages
            .iter()
            .filter(|package| package.direct && package.id.name() == &dependency.name);
        let Some(package) = candidates.next() else {
            return Err(SupplyChainError::MissingRootResolution {
                index: request_index,
                name: dependency.name.as_str().to_owned(),
            });
        };
        if candidates.next().is_some() {
            return Err(SupplyChainError::AmbiguousCachedRootResolution {
                index: request_index,
                name: dependency.name.as_str().to_owned(),
            });
        }
        roots.push(RootResolution {
            request_index,
            request_name: dependency.name.clone(),
            request_spec: dependency.range.as_str().to_owned(),
            package_id: package.id.clone(),
        });
    }
    Ok(roots)
}

/// Read exact root pins from the owner-scoped lock section, never from package rows by name.
/// Đọc pin root chính xác theo owner; không suy ra từ package row chỉ dựa trên tên.
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
