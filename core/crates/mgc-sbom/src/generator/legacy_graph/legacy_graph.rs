//! Legacy lockfile to CycloneDX projection — Chiếu lockfile legacy sang CycloneDX

use std::collections::HashMap;

use crate::cyclonedx::*;
use crate::generator::error::{GeneratorError, GeneratorResult};
use crate::generator::identity::{legacy_bom_ref, resolve_legacy_dependency_ref};
use crate::mapping::{component_name, package_purl};
use mgc_lockfile::Lockfile;

/// Generate SBOM from lockfile — Tạo SBOM từ lockfile
pub(crate) fn generate(lockfile: &Lockfile, include_hashes: bool) -> GeneratorResult<Bom> {
    let mut bom = Bom::new();

    let mut components = Vec::new();
    let mut dependency_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut refs_by_legacy_id: HashMap<String, Vec<String>> = HashMap::new();
    let mut refs_by_legacy_name: HashMap<String, Vec<String>> = HashMap::new();
    let mut unique_refs = std::collections::HashSet::new();

    for pkg in &lockfile.packages {
        let bom_ref = legacy_bom_ref(pkg);
        if !unique_refs.insert(bom_ref.clone()) {
            return Err(GeneratorError::InvalidLockfile(format!(
                "duplicate legacy package identity: {}@{} ({})",
                pkg.name, pkg.version, pkg.ecosystem
            )));
        }
        refs_by_legacy_id
            .entry(format!("{}@{}", pkg.name, pkg.version))
            .or_default()
            .push(bom_ref);
        refs_by_legacy_name
            .entry(pkg.name.clone())
            .or_default()
            .push(legacy_bom_ref(pkg));
    }

    for pkg in &lockfile.packages {
        let bom_ref = legacy_bom_ref(pkg);

        let mut component = Component {
            component_type: ComponentType::Library,
            bom_ref: bom_ref.clone(),
            name: component_name(pkg.ecosystem, &pkg.name),
            version: pkg.version.clone(),
            purl: package_purl(pkg.ecosystem, &pkg.name, &pkg.version),
            hashes: None,
            licenses: None,
        };

        // Add hashes when requested — Thêm hash khi được yêu cầu
        if include_hashes && !pkg.integrity.is_empty() {
            // Parse integrity values — Phân tích giá trị integrity như "blake3:abc123..."
            if let Some((alg, content)) = pkg.integrity.split_once(':') {
                component.hashes = Some(vec![Hash {
                    alg: alg.to_uppercase(),
                    content: content.to_string(),
                }]);
            }
        }

        components.push(component);

        if !pkg.dependencies.is_empty() {
            let deps = pkg
                .dependencies
                .iter()
                .map(|dependency| {
                    resolve_legacy_dependency_ref(
                        dependency,
                        &refs_by_legacy_id,
                        &refs_by_legacy_name,
                    )
                })
                .collect::<GeneratorResult<Vec<_>>>()?;
            dependency_map.insert(bom_ref, deps);
        }
    }

    bom.components = components;

    // Build the dependency graph — Dựng đồ thị dependency
    let dependencies: Vec<Dependency> = dependency_map
        .into_iter()
        .map(|(dep_ref, depends_on)| Dependency {
            dependency_ref: dep_ref,
            depends_on: Some(depends_on),
        })
        .collect();

    if !dependencies.is_empty() {
        bom.dependencies = Some(dependencies);
    }

    Ok(bom)
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
