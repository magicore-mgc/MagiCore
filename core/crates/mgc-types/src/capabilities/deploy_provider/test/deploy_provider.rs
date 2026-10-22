//! Deploy provider capability contract tests.
//! Test hợp đồng capability deploy provider.

use crate::capabilities::DeployProvider;
use crate::capabilities::test_support::{MockCore, assert_unsupported};

#[test]
fn default_probe_fails_closed_with_original_contract() {
    assert_unsupported(
        MockCore.probe_deploy().unwrap_err(),
        "web",
        "deploy",
        "this core has no deploy capability",
    );
}
