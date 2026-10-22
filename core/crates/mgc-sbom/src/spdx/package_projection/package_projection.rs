//! CycloneDX components to SPDX packages — Chiếu component CycloneDX sang package SPDX

use std::collections::BTreeMap;

use super::super::{
    SpdxPackage,
    checksum_projection::project_checksum,
    error::{SpdxError, SpdxResult},
};
use crate::Bom;

pub(in crate::spdx) fn project_packages(
    bom: &Bom,
    package_ids: &BTreeMap<String, String>,
) -> SpdxResult<Vec<SpdxPackage>> {
    let mut packages = Vec::with_capacity(bom.components.len());
    for component in &bom.components {
        let spdx_id = package_ids
            .get(&component.bom_ref)
            .cloned()
            .ok_or_else(|| SpdxError::PackageIdentityDisappeared(component.bom_ref.clone()))?;
        let checksums = component
            .hashes
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(project_checksum)
            .collect();
        let external_refs = component
            .purl
            .as_deref()
            .filter(|purl| purl.starts_with("pkg:") && !purl.chars().any(char::is_whitespace))
            .map(|purl| {
                vec![super::super::SpdxExternalRef {
                    reference_category: "PACKAGE-MANAGER",
                    reference_type: "purl",
                    reference_locator: purl.to_string(),
                }]
            })
            .unwrap_or_default();
        packages.push(SpdxPackage {
            spdx_id,
            name: component.name.clone(),
            version_info: (!component.version.is_empty()).then(|| component.version.clone()),
            download_location: "NOASSERTION",
            files_analyzed: false,
            license_concluded: "NOASSERTION",
            license_declared: "NOASSERTION",
            copyright_text: "NOASSERTION",
            checksums,
            external_refs,
        });
    }
    packages.sort_by(|left, right| left.spdx_id.cmp(&right.spdx_id));
    Ok(packages)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
