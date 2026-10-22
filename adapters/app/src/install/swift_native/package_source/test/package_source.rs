//! Behavior tests for the package_source owner.
//! Test hành vi cho owner package_source.

use super::super::*;
use mgc_types::{PackageId, ResolvedPackage};

fn package_with_url(url: &str) -> ResolvedPackage {
    ResolvedPackage {
        id: PackageId::parse("scope/lib@1.0.0").unwrap(),
        integrity: String::new(),
        tarball_url: url.to_string(),
        deps: Vec::new(),
        peer_deps: Vec::new(),
        direct: true,
        dev: false,
    }
}

#[test]
fn zip_suffix_selects_registry_archive() {
    assert_eq!(
        swift_package_source(&package_with_url("https://registry.test/scope/lib.zip")),
        SwiftPackageSource::RegistryArchive
    );
}

#[test]
fn non_zip_suffix_keeps_the_defensive_git_branch() {
    assert_eq!(
        swift_package_source(&package_with_url("https://git.test/scope/lib")),
        SwiftPackageSource::GitCheckout
    );
}

#[test]
fn zip_suffix_remains_case_sensitive() {
    assert_eq!(
        swift_package_source(&package_with_url("https://registry.test/scope/lib.ZIP")),
        SwiftPackageSource::GitCheckout
    );
}
