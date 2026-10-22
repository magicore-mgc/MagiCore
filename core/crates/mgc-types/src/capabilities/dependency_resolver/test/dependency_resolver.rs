//! Dependency resolver capability contract tests.
//! Test hợp đồng capability resolve dependency.

use crate::capabilities::DependencyResolver;
use crate::capabilities::test_support::{MockCore, assert_unsupported};
use crate::{Ecosystem, Manifest};
use futures_util::future::FutureExt;

#[test]
fn default_probe_fails_closed_with_original_contract() {
    assert_unsupported(
        MockCore.probe_dependency_resolver().unwrap_err(),
        "web",
        "resolve",
        "this core has no native registry graph for this project; package-manager fallback is disabled",
    );
}

#[test]
fn default_resolve_fails_closed_without_polling_the_network() {
    let result = MockCore
        .resolve(&Manifest::new("fixture", Ecosystem::Web))
        .now_or_never()
        .expect("default resolve must complete immediately");
    assert_unsupported(
        result.unwrap_err(),
        "web",
        "resolve",
        "this core has no native registry graph for this project; package-manager fallback is disabled",
    );
}
