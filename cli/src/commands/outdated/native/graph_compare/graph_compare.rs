//! Compares verified current pins to fresh native candidates.
//! So sánh pin hiện tại đã xác minh với candidate native mới.

use super::super::super::{error::OutdatedError, model::OutdatedPkg};

pub(in crate::commands::outdated::native) fn native_outdated_packages(
    manifest: &mgc_types::Manifest,
    current: &mgc_types::adapter::ResolvedGraph,
    latest: &mgc_types::adapter::ResolvedGraph,
) -> std::result::Result<Vec<OutdatedPkg>, OutdatedError> {
    let mut outdated = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for dependency in manifest.all_dependencies() {
        if !seen.insert(dependency.name.to_string()) {
            continue;
        }
        let current_matches = current
            .packages
            .iter()
            .filter(|package| package.direct && package.id.name() == &dependency.name)
            .collect::<Vec<_>>();
        let latest_matches = latest
            .packages
            .iter()
            .filter(|package| package.direct && package.id.name() == &dependency.name)
            .collect::<Vec<_>>();
        let [current_package] = current_matches.as_slice() else {
            return Err(OutdatedError::CurrentDirectCardinality {
                package: dependency.name.to_string(),
                found: current_matches.len(),
            });
        };
        let [latest_package] = latest_matches.as_slice() else {
            return Err(OutdatedError::LatestDirectCardinality {
                package: dependency.name.to_string(),
                found: latest_matches.len(),
            });
        };
        let current_version = current_package.id.version();
        let latest_version = latest_package.id.version();
        if latest_version < current_version {
            return Err(OutdatedError::Downgrade {
                package: dependency.name.to_string(),
                current: current_version.to_string(),
                latest: latest_version.to_string(),
            });
        }
        if latest_version > current_version {
            outdated.push(OutdatedPkg {
                name: dependency.name.to_string(),
                current: current_version.to_string(),
                latest: latest_version.to_string(),
                major: latest_version.major,
                minor: latest_version.minor,
                patch: latest_version.patch,
            });
        }
    }
    outdated.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(outdated)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
