//! `list.rs` — List installed web dependencies from materialized package manifests.

use mgc_types::adapter::InstalledPackage;
use mgc_types::{MgError, MgResult, PackageId, Version};
use std::path::Path;

use crate::lockfile::{installed_package_version, read_web_lockfile_checked};
use crate::manifest::parse_manifest;

pub async fn run_list(project_root: &Path) -> MgResult<Vec<InstalledPackage>> {
    let manifest = parse_manifest(project_root)?;
    // Dispatch verified V4 separately; never parse it through the legacy reader.
    // V4 vẫn qua verify digest/chữ ký, rồi chọn graph theo owner của project.
    let v4_graph = crate::lockfile::read_v4_document_if_present(project_root)?
        .map(|document| {
            let owner = crate::lockfile::web_lock_owner_core(project_root)?;
            mgc_lockfile::v4_graph::graph_from_v4_lockfile(&document, &owner)
                .map_err(|error| MgError::Other(error.to_string()))
        })
        .transpose()?;
    let lockfile = if v4_graph.is_some() {
        None
    } else {
        read_web_lockfile_checked(project_root)?
    };
    let node_modules = project_root.join("node_modules");
    let mut packages = Vec::new();

    for (label, deps) in manifest.dep_groups() {
        let is_dev = label == "devDependencies";
        for dep in deps {
            let path = node_modules.join(dep.name.as_str());
            if !path.exists() {
                continue;
            }

            let version = installed_package_version(&path).ok_or_else(|| MgError::Unsupported {
                core: "web",
                capability: "list installed package version",
                guidance: format!(
                    "cannot verify the installed version of '{}': its materialized package.json has no valid version; a lock pin alone is not proof of installed state",
                    dep.name.as_str()
                ),
            })?;

            let integrity = v4_graph
                .as_ref()
                .and_then(|graph| {
                    graph
                        .packages
                        .iter()
                        .find(|pkg| {
                            pkg.id.name_str() == dep.name.as_str() && pkg.id.version() == &version
                        })
                        .map(|pkg| pkg.integrity.clone())
                })
                .or_else(|| {
                    lockfile.as_ref().and_then(|lock| {
                        lock.packages
                            .iter()
                            .find(|pkg| {
                                pkg.name == dep.name.as_str()
                                    && Version::parse(&pkg.version).ok().as_ref() == Some(&version)
                            })
                            .map(|pkg| pkg.integrity.clone())
                    })
                });
            let is_direct = lockfile
                .as_ref()
                .map(|lock| {
                    lock.packages
                        .iter()
                        .find(|pkg| pkg.name == dep.name.as_str())
                        // direct field not in v2 schema - check if in manifest dependencies
                        .map(|pkg| {
                            manifest
                                .all_dependencies()
                                .any(|dep| dep.name.as_str() == pkg.name)
                        })
                        .unwrap_or(true)
                })
                .unwrap_or(true);

            packages.push(InstalledPackage {
                id: PackageId::new(dep.name.clone(), version),
                path,
                integrity,
                is_direct,
                is_dev,
            });
        }
    }

    Ok(packages)
}
