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
    // SEC-09: only the two canonical hosts are trusted now.
    // (SEC-09: giờ chỉ hai host chính thức được tin.)
    assert!(is_tarball_url_trusted(
        "https://registry.npmjs.org/pkg.tgz",
        "https://registry.npmjs.org/"
    ));
    assert!(is_tarball_url_trusted(
        "https://registry.yarnpkg.com/pkg.tgz",
        "https://registry.npmjs.org/"
    ));
}

/// SEC-09: any other `*.npmjs.org` host is not the public registry.
/// (SEC-09: mọi host `*.npmjs.org` khác không phải registry công khai.)
#[test]
fn rejects_other_npmjs_subdomains() {
    assert!(!is_tarball_url_trusted(
        "https://cdn.npmjs.org/pkg.tgz",
        "https://registry.npmjs.org/"
    ));
    assert!(!is_tarball_url_trusted(
        "https://evil.npmjs.org/pkg.tgz",
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

/// SEC-09: a loopback tarball is trusted only for a loopback registry.
/// (SEC-09: tarball loopback chỉ được tin khi registry cũng loopback.)
#[test]
fn loopback_tarball_requires_a_loopback_registry() {
    assert!(is_tarball_url_trusted(
        "http://127.0.0.1:4315/pkg.tgz",
        "http://127.0.0.1:4315/"
    ));
    assert!(!is_tarball_url_trusted(
        "http://127.0.0.1:4315/pkg.tgz",
        "https://registry.example.test/"
    ));
    assert!(!is_tarball_url_trusted(
        "http://localhost/pkg.tgz",
        "https://registry.npmjs.org/"
    ));
}
