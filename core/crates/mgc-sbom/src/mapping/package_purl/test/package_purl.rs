#![allow(clippy::unwrap_used)]
//! Package URL mapping tests — Kiểm thử ánh xạ Package URL

use super::super::*;

#[test]
fn maps_pypi_names_and_rejects_uppercase_go_modules() {
    assert_eq!(
        package_purl(EcosystemTag::Python, "Django_REST.framework", "3.15.0",).as_deref(),
        Some("pkg:pypi/django-rest-framework@3.15.0"),
    );
    assert_eq!(
        package_purl(EcosystemTag::Go, "Example.com/Lib", "1.0.0"),
        None
    );
}
