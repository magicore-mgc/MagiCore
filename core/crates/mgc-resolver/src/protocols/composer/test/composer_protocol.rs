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
    // The shared semver range syntax disagrees with Composer's caret rule
    // for `0.x`, which proves the trait override is the one in effect.
    // Cú pháp range semver dùng chung lệch quy tắc caret `0.x` của Composer,
    // chứng minh override của trait đang có hiệu lực.
    let zero = Version::parse("0.3.9").expect("test version");
    assert!(protocol.matches_requirement("^0.3", &zero));
    let range = VersionRange::parse("^0.3").expect("test range");
    assert!(!range.matches(&zero));
}
