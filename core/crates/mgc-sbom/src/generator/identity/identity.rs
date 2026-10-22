//! Stable SBOM references without private source disclosure — Tạo ref ổn định, không lộ nguồn riêng tư

use std::collections::HashMap;

use crate::generator::error::{GeneratorError, GeneratorResult};
use crate::mapping::percent_encode_path;
use mgc_lockfile::{schema::Package, v4::PackageKey};

pub(crate) fn legacy_bom_ref(package: &Package) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"mgc-sbom-legacy-identity-v1\0");
    for value in [&package.name, &package.version, &package.resolved] {
        hasher.update(&(value.len() as u64).to_be_bytes());
        hasher.update(value.as_bytes());
    }
    format!(
        "mgc:lock-v3:{}:{}:{}:{}",
        package.ecosystem,
        percent_encode_path(&package.name),
        percent_encode_path(&package.version),
        hasher.finalize().to_hex()
    )
}

pub(crate) fn v4_bom_ref(key: &PackageKey) -> GeneratorResult<String> {
    // Hash PackageKey because it may contain private source data.
    // Băm PackageKey vì nó có thể chứa thông tin nguồn riêng tư.
    let identity = serde_json::to_vec(key)?;
    Ok(format!("mgc:lock-v4:{}", blake3::hash(&identity).to_hex()))
}

pub(crate) fn resolve_legacy_dependency_ref(
    dependency: &str,
    refs_by_legacy_id: &HashMap<String, Vec<String>>,
    refs_by_legacy_name: &HashMap<String, Vec<String>>,
) -> GeneratorResult<String> {
    let refs = refs_by_legacy_id
        .get(dependency)
        .or_else(|| {
            // Accept a bare name only when it identifies one locked package.
            // Chỉ nhận tên trần khi nó khớp duy nhất một package trong graph.
            refs_by_legacy_name.get(dependency)
        })
        .ok_or_else(|| {
            GeneratorError::InvalidLockfile(format!(
                "dependency '{dependency}' does not resolve to a package in mgc.lock"
            ))
        })?;
    if refs.len() != 1 {
        return Err(GeneratorError::InvalidLockfile(format!(
            "dependency '{dependency}' is ambiguous in legacy mgc.lock"
        )));
    }
    Ok(refs[0].clone())
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
