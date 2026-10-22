//! Check dependency/peer reachability and reject owner-scoped orphan lock rows.
//! Kiểm tra đường đi dependency/peer và từ chối lock row mồ côi theo owner.

use std::collections::HashSet;

use mgc_lockfile::Lockfile;
use mgc_types::PackageId;

use super::super::package_index::{
    OwnerPackageIndex, belongs_to_owner, scoped_name_candidates, scoped_package_candidates,
};
use super::super::root_selection::SelectedRoots;

pub(crate) fn reachable_closure_matches(
    lock: &Lockfile,
    package_index: &OwnerPackageIndex,
    roots: SelectedRoots,
) -> bool {
    let mut pending = roots.pending;
    let mut reachable = HashSet::new();
    while let Some(index) = pending.pop() {
        if !reachable.insert(index) {
            continue;
        }
        let package = &lock.packages[index];
        let references = package
            .dependencies
            .iter()
            .chain(package.peers.iter().flatten());
        for reference in references {
            let candidates = if let Ok(id) = PackageId::parse(reference) {
                scoped_package_candidates(
                    package_index,
                    package.owner_core.clone(),
                    package.ecosystem,
                    id.name_str().to_string(),
                    id.version().to_string(),
                )
            } else {
                scoped_name_candidates(
                    package_index,
                    package.owner_core.clone(),
                    package.ecosystem,
                    reference.clone(),
                )
            };
            let Some([target]) = candidates else {
                return false;
            };
            pending.push(*target);
        }
    }

    let expected_packages = lock
        .packages
        .iter()
        .enumerate()
        .filter(|(index, package)| {
            belongs_to_owner(package_index, lock, *index)
                && roots.selected_ecosystems.contains(&package.ecosystem)
        })
        .count();

    reachable.len() == expected_packages
}
