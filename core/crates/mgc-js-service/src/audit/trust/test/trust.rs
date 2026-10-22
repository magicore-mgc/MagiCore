// Trust-policy boundary tests. / Test ranh giới policy trust.
#![allow(clippy::unwrap_used)]

use super::is_tarball_url_trusted;

#[test]
fn accepts_same_registry_host() {
    assert!(is_tarball_url_trusted(
        "https://registry.example.test/pkg.tgz",
        "https://registry.example.test/"
    ));
}

#[test]
fn accepts_npmjs_distribution_hosts() {
    assert!(is_tarball_url_trusted(
        "https://cdn.npmjs.org/pkg.tgz",
        "https://registry.npmjs.org/"
    ));
    assert!(is_tarball_url_trusted(
        "https://registry.yarnpkg.com/pkg.tgz",
        "https://registry.npmjs.org/"
    ));
}

#[test]
fn rejects_deceptive_npmjs_suffix() {
    assert!(!is_tarball_url_trusted(
        "https://registry.npmjs.org.evil.invalid/pkg.tgz",
        "https://registry.npmjs.org/"
    ));
}

#[test]
fn rejects_malformed_urls() {
    assert!(!is_tarball_url_trusted(
        "not a URL",
        "https://registry.npmjs.org/"
    ));
    assert!(!is_tarball_url_trusted(
        "https://registry.npmjs.org/pkg.tgz",
        "not a URL"
    ));
}

#[test]
fn preserves_explicit_loopback_behavior() {
    assert!(is_tarball_url_trusted(
        "http://127.0.0.1:4315/pkg.tgz",
        "https://registry.example.test/"
    ));
}
