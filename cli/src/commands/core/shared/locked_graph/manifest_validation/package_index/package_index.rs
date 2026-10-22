//! Build owner- and ecosystem-scoped indexes for lock rows.
//! Dựng index lock theo owner và ecosystem.

use std::collections::HashMap;

use mgc_lockfile::{EcosystemTag, Lockfile};

type ScopedName = (Option<String>, EcosystemTag, String);
type ScopedPackage = (Option<String>, EcosystemTag, String, String);
type OwnerPackage = (Option<String>, String, String);

pub(crate) struct OwnerPackageIndex {
    owner_core: String,
    has_foreign_owner: bool,
    by_name: HashMap<String, Vec<usize>>,
    by_scoped_name: HashMap<ScopedName, Vec<usize>>,
    by_scoped_package: HashMap<ScopedPackage, Vec<usize>>,
    by_owner_package: HashMap<OwnerPackage, Vec<usize>>,
}

impl OwnerPackageIndex {
    pub(super) fn new(lock: &Lockfile, owner_core: &str) -> Self {
        let has_foreign_owner = lock.packages.iter().any(|package| {
            package
                .owner_core
                .as_deref()
                .is_some_and(|owner| owner != owner_core)
        });
        let mut by_name = HashMap::new();
        let mut by_scoped_name = HashMap::new();
        let mut by_scoped_package = HashMap::new();
        let mut by_owner_package = HashMap::new();

        for (index, package) in lock.packages.iter().enumerate() {
            by_name
                .entry(package.name.clone())
                .or_insert_with(Vec::new)
                .push(index);
            by_scoped_name
                .entry((
                    package.owner_core.clone(),
                    package.ecosystem,
                    package.name.clone(),
                ))
                .or_insert_with(Vec::new)
                .push(index);
            by_scoped_package
                .entry((
                    package.owner_core.clone(),
                    package.ecosystem,
                    package.name.clone(),
                    package.version.clone(),
                ))
                .or_insert_with(Vec::new)
                .push(index);
            by_owner_package
                .entry((
                    package
                        .owner_core
                        .clone()
                        .or_else(|| (!has_foreign_owner).then(|| owner_core.to_string())),
                    package.name.clone(),
                    package.version.clone(),
                ))
                .or_insert_with(Vec::new)
                .push(index);
        }

        Self {
            owner_core: owner_core.to_string(),
            has_foreign_owner,
            by_name,
            by_scoped_name,
            by_scoped_package,
            by_owner_package,
        }
    }

    pub(crate) fn owner_core(&self) -> &str {
        &self.owner_core
    }

    pub(crate) fn belongs_to_owner(&self, lock: &Lockfile, index: usize) -> bool {
        let owner = lock.packages[index].owner_core.as_deref();
        owner == Some(self.owner_core.as_str()) || (owner.is_none() && !self.has_foreign_owner)
    }

    pub(crate) fn has_owner_packages(&self, lock: &Lockfile) -> bool {
        lock.packages
            .iter()
            .enumerate()
            .any(|(index, _)| self.belongs_to_owner(lock, index))
    }

    pub(crate) fn name_candidates(&self, name: &str) -> &[usize] {
        self.by_name.get(name).map_or(&[], Vec::as_slice)
    }

    pub(crate) fn owner_package_candidates(&self, name: &str, version: &str) -> &[usize] {
        self.by_owner_package
            .get(&(
                Some(self.owner_core.clone()),
                name.to_string(),
                version.to_string(),
            ))
            .map_or(&[], Vec::as_slice)
    }

    pub(crate) fn scoped_name_candidates(
        &self,
        owner_core: Option<String>,
        ecosystem: EcosystemTag,
        name: String,
    ) -> Option<&[usize]> {
        self.by_scoped_name
            .get(&(owner_core, ecosystem, name))
            .map(Vec::as_slice)
    }

    pub(crate) fn scoped_package_candidates(
        &self,
        owner_core: Option<String>,
        ecosystem: EcosystemTag,
        name: String,
        version: String,
    ) -> Option<&[usize]> {
        self.by_scoped_package
            .get(&(owner_core, ecosystem, name, version))
            .map(Vec::as_slice)
    }
}

pub(crate) fn build_owner_package_index(lock: &Lockfile, owner_core: &str) -> OwnerPackageIndex {
    OwnerPackageIndex::new(lock, owner_core)
}

pub(crate) fn owner_core(index: &OwnerPackageIndex) -> &str {
    index.owner_core()
}

pub(crate) fn belongs_to_owner(
    index: &OwnerPackageIndex,
    lock: &Lockfile,
    package_index: usize,
) -> bool {
    index.belongs_to_owner(lock, package_index)
}

pub(crate) fn has_owner_packages(index: &OwnerPackageIndex, lock: &Lockfile) -> bool {
    index.has_owner_packages(lock)
}

pub(crate) fn name_candidates<'a>(index: &'a OwnerPackageIndex, name: &str) -> &'a [usize] {
    index.name_candidates(name)
}

pub(crate) fn owner_package_candidates<'a>(
    index: &'a OwnerPackageIndex,
    name: &str,
    version: &str,
) -> &'a [usize] {
    index.owner_package_candidates(name, version)
}

pub(crate) fn scoped_name_candidates(
    index: &OwnerPackageIndex,
    owner_core: Option<String>,
    ecosystem: EcosystemTag,
    name: String,
) -> Option<&[usize]> {
    index.scoped_name_candidates(owner_core, ecosystem, name)
}

pub(crate) fn scoped_package_candidates(
    index: &OwnerPackageIndex,
    owner_core: Option<String>,
    ecosystem: EcosystemTag,
    name: String,
    version: String,
) -> Option<&[usize]> {
    index.scoped_package_candidates(owner_core, ecosystem, name, version)
}
