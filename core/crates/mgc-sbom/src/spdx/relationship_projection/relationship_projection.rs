//! CycloneDX dependencies to deterministic SPDX relationships — Chiếu dependency sang relationship SPDX xác định

use std::collections::{BTreeMap, BTreeSet};

use super::super::{
    SpdxRelationship,
    error::{SpdxError, SpdxResult},
};
use crate::Bom;

pub(in crate::spdx) fn project_relationships(
    bom: &Bom,
    package_ids: &BTreeMap<String, String>,
    sorted_packages: &[super::super::SpdxPackage],
) -> SpdxResult<(Vec<String>, Vec<SpdxRelationship>)> {
    let mut relationship_set = BTreeSet::new();
    let mut document_describes = Vec::with_capacity(sorted_packages.len());
    for package in sorted_packages {
        document_describes.push(package.spdx_id.clone());
        relationship_set.insert(SpdxRelationship {
            spdx_element_id: "SPDXRef-DOCUMENT".to_string(),
            relationship_type: "DESCRIBES",
            related_spdx_element: package.spdx_id.clone(),
        });
    }
    for dependency in bom.dependencies.as_deref().unwrap_or_default() {
        let source = package_ids
            .get(&dependency.dependency_ref)
            .ok_or_else(|| SpdxError::DependencySourceMissing(dependency.dependency_ref.clone()))?;
        for target_ref in dependency.depends_on.as_deref().unwrap_or_default() {
            let target = package_ids
                .get(target_ref)
                .ok_or_else(|| SpdxError::DependencyTargetMissing(target_ref.clone()))?;
            relationship_set.insert(SpdxRelationship {
                spdx_element_id: source.clone(),
                relationship_type: "DEPENDS_ON",
                related_spdx_element: target.clone(),
            });
        }
    }
    Ok((document_describes, relationship_set.into_iter().collect()))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
