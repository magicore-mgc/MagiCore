#![allow(clippy::unwrap_used)]
//! Tarball URL owner contract tests. / Kiểm thử contract owner URL tarball.

use super::super::*;
use mgc_types::{PackageId, adapter::ResolvedPackage};

fn package_with_url(tarball_url: &str) -> ResolvedPackage {
    ResolvedPackage {
        id: PackageId::parse("react@18.3.1").unwrap(),
        integrity: String::new(),
        tarball_url: tarball_url.to_string(),
        deps: Vec::new(),
        peer_deps: Vec::new(),
        direct: true,
        dev: false,
    }
}

#[test]
fn missing_tarball_url_uses_registry_fallback() {
    let package = package_with_url("");

    assert_eq!(
        package_tarball_url("https://registry.example/", &package),
        "https://registry.example/react/-/react-18.3.1.tgz"
    );
}

#[test]
fn untrusted_insecure_tarball_url_uses_registry_fallback() {
    let package = package_with_url("http://packages.invalid/react.tgz");

    assert_eq!(
        package_tarball_url("https://registry.example", &package),
        "https://registry.example/react/-/react-18.3.1.tgz"
    );
}
