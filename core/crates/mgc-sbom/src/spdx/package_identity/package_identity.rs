//! Stable SPDX package IDs and identity validation — ID package SPDX ổn định và xác thực identity

use std::collections::BTreeMap;

use super::super::error::{SpdxError, SpdxResult};
use crate::Bom;

pub(in crate::spdx) fn build_package_ids(bom: &Bom) -> SpdxResult<BTreeMap<String, String>> {
    let mut package_ids = BTreeMap::new();
    for component in &bom.components {
        if component.name.trim().is_empty() {
            return Err(SpdxError::EmptyPackageName);
        }
        let spdx_id = package_spdx_id(&component.bom_ref);
        if package_ids
            .insert(component.bom_ref.clone(), spdx_id)
            .is_some()
        {
            return Err(SpdxError::DuplicatePackageIdentity(
                component.bom_ref.clone(),
            ));
        }
    }
    Ok(package_ids)
}

fn package_spdx_id(bom_ref: &str) -> String {
    format!(
        "SPDXRef-Package-{}",
        blake3::hash(bom_ref.as_bytes()).to_hex()
    )
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
