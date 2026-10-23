//! Prior direct-pin reading, satisfaction checks and root-pin parsing.
//! Đọc pin trực tiếp trước đó, kiểm thoả mãn và parse root pin.

use std::collections::HashMap;
use std::path::Path;

use crate::provider::NpmDependencyProvider;
use mgc_types::{Manifest, PackageId, PackageName, Version, VersionRange};

use crate::supply_chain::error::SupplyChainError;

pub(crate) fn read_previous_direct_pins(
    project_root: &Path,
    manifest: &Manifest,
) -> Result<HashMap<usize, Version>, SupplyChainError> {
    let owner = crate::lockfile::web_lock_owner_core(project_root)
        .map_err(|source| SupplyChainError::LockfileRead { source })?;
    let Some(mut lockfile) = crate::lockfile::read_web_lockfile_document(project_root)
        .map_err(|source| SupplyChainError::LockfileRead { source })?
    else {
        return Ok(HashMap::new());
    };
    crate::lockfile::scope_web_lock_packages(&mut lockfile, &owner);

    let root_pins = if lockfile.version == "2" && owner == "web" {
        Some(lockfile.root_dependencies.as_slice())
    } else {
        lockfile
            .root_dependencies_by_owner
            .get(&owner)
            .map(Vec::as_slice)
    };
    let Some(root_pins) = root_pins else {
        // Older Web locks may have package rows but no root identity. Such rows
        // cannot safely establish a direct baseline.
        // Lock Web cũ có thể chỉ có package row; không thể suy ra pin direct an toàn.
        return Ok(HashMap::new());
    };

    let mut root_packages = Vec::with_capacity(root_pins.len());
    for encoded_pin in root_pins {
        if let Some(package_id) = parse_web_root_pin(encoded_pin)? {
            let version = package_id.version().to_string();
            let package_exists = lockfile.packages.iter().any(|package| {
                let owned_web_row = package.ecosystem == mgc_lockfile::EcosystemTag::Web
                    && package.owner_core.as_deref() == Some(owner.as_str());
                let legacy_web_v2_row = lockfile.version == "2"
                    && owner == "web"
                    && package.ecosystem == mgc_lockfile::EcosystemTag::Other
                    && package.owner_core.is_none();
                (owned_web_row || legacy_web_v2_row)
                    && package.name == package_id.name_str()
                    && package.version == version
            });
            if !package_exists {
                return Err(SupplyChainError::MissingLockedRootPackage {
                    package_id: package_id.to_string(),
                });
            }
            root_packages.push(package_id);
        }
    }

    let mut previous_top = HashMap::new();
    for (request_index, dependency) in manifest.all_dependencies().enumerate() {
        let mut matches = root_packages.iter().filter(|package_id| {
            package_id.name() == &dependency.name
                && previous_root_pin_satisfies_request(dependency, package_id.version())
        });
        let Some(root_package) = matches.next() else {
            continue;
        };
        if matches.next().is_some() {
            return Err(SupplyChainError::AmbiguousDirectRootPins {
                name: dependency.name.as_str().to_owned(),
            });
        }
        previous_top.insert(request_index, root_package.version().clone());
    }

    Ok(previous_top)
}

/// Keep a moving registry tag's prior pin as a baseline; semver ranges still reject stale pins.
/// Giữ pin cũ làm baseline cho registry tag thay đổi; semver range vẫn loại pin stale.
fn previous_root_pin_satisfies_request(
    dependency: &mgc_types::DependencySpec,
    version: &Version,
) -> bool {
    let raw_spec = dependency.range.as_str();
    let normalized_spec = NpmDependencyProvider::parse_alias_spec(raw_spec)
        .map(|(_, range)| range)
        .unwrap_or_else(|| raw_spec.to_owned());
    if mgc_js_service::resolve::is_dist_tag_spec(&normalized_spec) {
        return true;
    }

    VersionRange::parse(&normalized_spec).is_ok_and(|range| range.matches(version))
}

/// Parse only Web and legacy owner-local pins; reject unknown qualified pin encodings.
/// Chỉ nhận pin Web hoặc pin owner-local cũ; từ chối encoding có qualifier không nhận diện.
fn parse_web_root_pin(encoded_pin: &str) -> Result<Option<PackageId>, SupplyChainError> {
    const ROOT_PIN_PREFIX: &str = "mgc-root-v1:";

    let root_pin = mgc_lockfile::parse_root_pin(encoded_pin);
    match root_pin.ecosystem {
        Some(ecosystem) if ecosystem != mgc_lockfile::EcosystemTag::Web => return Ok(None),
        None if encoded_pin.starts_with(ROOT_PIN_PREFIX) => {
            return Err(SupplyChainError::InvalidRootPin {
                pin: encoded_pin.to_owned(),
                source: mgc_types::MgError::InvalidPackageSpec(encoded_pin.to_owned()),
            });
        }
        _ => {}
    }

    let Some((name, raw_version)) = root_pin.package_id.rsplit_once('@') else {
        return Err(SupplyChainError::InvalidRootPin {
            pin: encoded_pin.to_owned(),
            source: mgc_types::MgError::InvalidPackageSpec(root_pin.package_id.to_owned()),
        });
    };
    let package_name =
        PackageName::new(name).map_err(|source| SupplyChainError::InvalidRootPin {
            pin: encoded_pin.to_owned(),
            source,
        })?;
    let version =
        Version::parse(raw_version).map_err(|source| SupplyChainError::InvalidLockfileVersion {
            name: package_name.as_str().to_owned(),
            version: raw_version.to_owned(),
            source: Box::new(source),
        })?;

    Ok(Some(PackageId::new(package_name, version)))
}
