//! Public protocol construction contract tests.
//! Test contract khởi tạo protocol public.

use crate::protocols::RegistryProtocol;
use crate::protocols::composer::ComposerProtocol;
use mgc_types::{Version, VersionRange};

#[test]
fn explicit_repository_url_trims_trailing_slashes() {
    let protocol = ComposerProtocol::new("https://repo.packagist.org///");
    assert_eq!(protocol.repository_url, "https://repo.packagist.org");
    assert_eq!(
        protocol.metadata_url("monolog/monolog"),
        "https://repo.packagist.org/p2/monolog/monolog.json"
    );
}

#[test]
fn environment_constructor_never_yields_an_empty_repository() {
    let protocol = ComposerProtocol::from_env();
    assert!(!protocol.repository_url.is_empty());
    assert!(!protocol.repository_url.ends_with('/'));
}

#[test]
fn trait_matches_requirement_uses_composer_semantics() {
    let protocol = ComposerProtocol::new("https://repo.packagist.org");
    let version = Version::parse("3.12.1").expect("test version");
    assert!(protocol.matches_requirement("^3.0", &version));
    assert!(protocol.matches_requirement("^3.10 || ^4", &version));
    // Composer pads a short component (`^3.10` → `^3.10.0`), while the shared
    // semver matcher reads a bare `3.10` term as a wildcard; the override is
    // therefore observable, and `composer_to_semver` covers the bridge back.
    // Composer bù thành phần thiếu, còn semver đọc `3.10` trần là wildcard,
    // nên override quan sát được; `composer_to_semver` phụ trách cầu nối.
    assert!(
        !VersionRange::parse("3.10")
            .expect("test range")
            .matches(&version)
    );
    let padded = VersionRange::parse("=3.10.0").expect("test range");
    assert!(!padded.matches(&version));
    assert!(!protocol.matches_requirement("=3.10.0", &version));
}
