//! Lifecycle runner capability contract tests.
//! Test hợp đồng capability lifecycle runner.

use crate::capabilities::LifecycleRunner;
use crate::capabilities::test_support::{MockCore, assert_unsupported};

#[test]
fn default_probe_fails_closed_with_original_contract() {
    assert_unsupported(
        MockCore.probe_lifecycle_runner().unwrap_err(),
        "web",
        "lifecycle",
        "this core has no mgc-run lifecycle phase",
    );
}
