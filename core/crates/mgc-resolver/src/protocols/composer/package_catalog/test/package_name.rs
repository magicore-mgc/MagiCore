//! Composer package-name validation tests.
//! Test kiểm tra tên package Composer.

use crate::protocols::composer::package_catalog::validate_composer_package_name;
use mgc_types::MgError;

#[test]
fn valid_names_are_lowercased() {
    assert_eq!(
        validate_composer_package_name("Monolog/Monolog").expect("valid Composer name"),
        "monolog/monolog"
    );
    assert_eq!(
        validate_composer_package_name("  psr/log ").expect("valid Composer name"),
        "psr/log"
    );
    assert_eq!(
        validate_composer_package_name("symfony/polyfill-php80").expect("valid Composer name"),
        "symfony/polyfill-php80"
    );
}

#[test]
fn invalid_names_fail_closed() {
    for name in [
        "",
        "monolog",
        "monolog/",
        "/monolog",
        "a/b/c",
        "mono log/log",
        "-vendor/package",
        "vendor/.package",
    ] {
        let error = validate_composer_package_name(name)
            .expect_err("invalid Composer name must be rejected");
        assert!(
            matches!(error, MgError::InvalidPackageName(_)),
            "{name}: {error:?}"
        );
    }
}
