//! Scaffold provider capability contract tests.
//! Test hợp đồng capability scaffold provider.

use crate::capabilities::ScaffoldProvider;
use crate::capabilities::test_support::{MockCore, assert_unsupported};

#[test]
fn default_probe_fails_closed_with_original_contract() {
    assert_unsupported(
        MockCore.probe_scaffold().unwrap_err(),
        "web",
        "scaffold",
        "this core does not implement scaffolding",
    );
}
