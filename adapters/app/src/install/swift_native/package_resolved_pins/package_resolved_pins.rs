//! Project each resolved Swift package into its Package.resolved pin shape.
//! Chiếu package Swift đã resolve sang shape pin Package.resolved.

use super::super::{SwiftPackageSource, git_commit_for_package, swift_package_source};
use super::{build_git_pin, build_registry_pin};
use mgc_resolver::protocols::swift::SwiftResolvedPin;
use mgc_types::ResolvedGraph;

/// Preserve package order while selecting the existing registry or Git pin projection.
/// Giữ thứ tự package và chọn projection registry hoặc Git hiện tại.
pub(in crate::install) fn build_swift_pins(
    graph: &ResolvedGraph,
    lock_packages: &[mgc_lockfile::Package],
) -> Vec<SwiftResolvedPin> {
    graph
        .packages
        .iter()
        .map(|package| {
            let name = package.id.name_str();
            match swift_package_source(package) {
                SwiftPackageSource::GitCheckout => build_git_pin(
                    name,
                    package.id.version(),
                    git_commit_for_package(lock_packages, name),
                ),
                SwiftPackageSource::RegistryArchive => {
                    build_registry_pin(name, package.id.version())
                }
            }
        })
        .collect()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
