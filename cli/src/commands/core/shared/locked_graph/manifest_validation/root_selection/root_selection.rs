//! Validate explicit or legacy lock roots against active manifest dependencies.
//! Đối chiếu root lock tường minh hoặc legacy với dependency của manifest hiện tại.

use std::collections::HashSet;

use mgc_lockfile::{Lockfile, parse_root_pin};
use mgc_types::{Manifest, PackageId, Version};

use super::super::package_index::{
    OwnerPackageIndex, belongs_to_owner, has_owner_packages, name_candidates, owner_core,
    owner_package_candidates,
};

pub(crate) enum ManifestRootSelection {
    Empty,
    Selected(SelectedRoots),
}

pub(crate) struct SelectedRoots {
    pub(crate) pending: Vec<usize>,
    pub(crate) selected_ecosystems: HashSet<mgc_lockfile::EcosystemTag>,
}

pub(crate) fn select_manifest_roots(
    lock: &Lockfile,
    manifest: &Manifest,
    package_index: &OwnerPackageIndex,
) -> Option<ManifestRootSelection> {
    let owner_core = owner_core(package_index);
    let explicit_roots = lock.root_dependencies_by_owner.get(owner_core);
    let mut root_records = Vec::<(usize, mgc_lockfile::EcosystemTag)>::new();

    if let Some(roots) = explicit_roots {
        let mut seen = HashSet::new();
        for root in roots {
            let parsed_root = parse_root_pin(root);
            let Ok(id) = PackageId::parse(parsed_root.package_id) else {
                return None;
            };
            let candidates =
                owner_package_candidates(package_index, id.name_str(), &id.version().to_string())
                    .iter()
                    .copied()
                    .filter(|index| belongs_to_owner(package_index, lock, *index))
                    .filter(|index| {
                        parsed_root
                            .ecosystem
                            .is_none_or(|ecosystem| lock.packages[*index].ecosystem == ecosystem)
                    })
                    .collect::<Vec<_>>();
            let [index] = candidates.as_slice() else {
                return None;
            };
            if !seen.insert(*index) {
                return None;
            }
            root_records.push((*index, lock.packages[*index].ecosystem));
        }
    }

    let mut pending = Vec::new();
    let mut expected_roots = HashSet::new();
    for dependency in manifest.all_dependencies() {
        let candidates: Vec<_> = if explicit_roots.is_some() {
            root_records
                .iter()
                .map(|(index, _)| *index)
                .filter(|index| lock.packages[*index].name == dependency.name.as_str())
                .filter(|index| {
                    Version::parse(&lock.packages[*index].version)
                        .is_ok_and(|version| dependency.range.matches(&version))
                })
                .collect()
        } else {
            name_candidates(package_index, dependency.name.as_str())
                .iter()
                .copied()
                .filter(|index| belongs_to_owner(package_index, lock, *index))
                .filter(|index| {
                    Version::parse(&lock.packages[*index].version)
                        .is_ok_and(|version| dependency.range.matches(&version))
                })
                .collect()
        };
        if candidates.len() != 1 {
            return None;
        }
        let index = candidates[0];
        expected_roots.insert(index);
        pending.push(index);
    }

    let selected_ecosystems = expected_roots
        .iter()
        .map(|index| lock.packages[*index].ecosystem)
        .collect::<HashSet<_>>();
    if selected_ecosystems.is_empty() {
        return if has_owner_packages(package_index, lock) {
            None
        } else {
            Some(ManifestRootSelection::Empty)
        };
    }
    if explicit_roots.is_some()
        && root_records.iter().any(|(index, ecosystem)| {
            selected_ecosystems.contains(ecosystem) && !expected_roots.contains(index)
        })
    {
        return None;
    }

    Some(ManifestRootSelection::Selected(SelectedRoots {
        pending,
        selected_ecosystems,
    }))
}
