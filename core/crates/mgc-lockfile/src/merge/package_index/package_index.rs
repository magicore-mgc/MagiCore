// Package identity indexing for 3-way lockfile merge.
// Index identity package cho merge lockfile ba chiều.
use std::collections::BTreeMap;

use crate::{Package, merge::MergeConflict};

pub(in crate::merge) type PackageIdentity = (Option<String>, String, String);
pub(in crate::merge) type PkgMap = BTreeMap<PackageIdentity, Package>;

pub(in crate::merge) fn index_pkgs(pkgs: &[Package]) -> Result<PkgMap, MergeConflict> {
    let mut indexed = PkgMap::new();
    for package in pkgs {
        let identity = (
            package.owner_core.clone(),
            package.ecosystem.as_str().to_string(),
            package.name.clone(),
        );
        if let Some(previous) = indexed.get(&identity) {
            if previous != package {
                // Multiple versions are not representable as one v3 merge value.
                // Nhiều version không thể gộp thành một giá trị merge v3.
                return Err(MergeConflict {
                    name: package.name.clone(),
                    base_version: None,
                    ours_version: previous.version.clone(),
                    theirs_version: package.version.clone(),
                });
            }
            continue;
        }
        indexed.insert(identity, package.clone());
    }
    Ok(indexed)
}
