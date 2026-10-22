//! Verified v4 graph to CycloneDX projection — Chiếu graph v4 đã xác minh sang CycloneDX

use std::collections::HashMap;

use crate::cyclonedx::*;
use crate::generator::error::{GeneratorError, GeneratorResult};
use crate::generator::identity::v4_bom_ref;
use crate::mapping::{component_name, is_hex_digest, package_purl};
use mgc_lockfile::{canonical::LockfileV4, policy::LockPolicyMode};

pub(crate) fn generate_with_report(
    lockfile: &LockfileV4,
    policy: LockPolicyMode,
    trust_keys: &[String],
    include_hashes: bool,
) -> GeneratorResult<(Bom, mgc_lockfile::policy::V4VerifyReport)> {
    if lockfile.version != mgc_lockfile::v4::LOCKFILE_SCHEMA_V4 {
        return Err(GeneratorError::InvalidLockfile(format!(
            "expected lockfile schema v4, got '{}'",
            lockfile.version
        )));
    }
    let report = mgc_lockfile::policy::verify_v4_math(lockfile)?;
    mgc_lockfile::policy::enforce_policy(&report, policy, trust_keys)?;
    let mut bom = Bom::new();
    let mut refs_by_key = HashMap::with_capacity(lockfile.packages.len());
    for package in &lockfile.packages {
        let bom_ref = v4_bom_ref(&package.key)?;
        if refs_by_key.insert(package.key.clone(), bom_ref).is_some() {
            return Err(GeneratorError::InvalidLockfile(format!(
                "duplicate v4 package identity: {}@{} ({})",
                package.key.name, package.key.version, package.key.ecosystem
            )));
        }
    }

    let mut components = Vec::with_capacity(lockfile.packages.len());
    let mut dependencies = Vec::new();
    for package in &lockfile.packages {
        let bom_ref = refs_by_key
            .get(&package.key)
            .ok_or_else(|| {
                GeneratorError::InvalidLockfile(
                    "package identity disappeared while building SBOM".to_string(),
                )
            })?
            .clone();
        let mut component = Component {
            component_type: ComponentType::Library,
            bom_ref: bom_ref.clone(),
            name: component_name(package.key.ecosystem, &package.key.name),
            version: package.key.version.clone(),
            purl: package_purl(
                package.key.ecosystem,
                &package.key.name,
                &package.key.version,
            ),
            hashes: None,
            licenses: None,
        };
        if include_hashes
            && let Some(artifact) = &package.artifact
            && is_hex_digest(&artifact.content_hash)
        {
            component.hashes = Some(vec![Hash {
                alg: "BLAKE3".to_string(),
                content: artifact.content_hash.clone(),
            }]);
        }
        components.push(component);

        if !package.edges.is_empty() {
            let mut depends_on = Vec::with_capacity(package.edges.len());
            for edge in &package.edges {
                let target = refs_by_key.get(&edge.target_key).ok_or_else(|| {
                    GeneratorError::InvalidLockfile(format!(
                        "dependency edge from {}@{} targets an absent locked package {}@{}",
                        package.key.name,
                        package.key.version,
                        edge.target_key.name,
                        edge.target_key.version
                    ))
                })?;
                depends_on.push(target.clone());
            }
            depends_on.sort();
            depends_on.dedup();
            dependencies.push(Dependency {
                dependency_ref: bom_ref,
                depends_on: Some(depends_on),
            });
        }
    }
    bom.components = components;
    if !dependencies.is_empty() {
        bom.dependencies = Some(dependencies);
    }
    Ok((bom, report))
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
