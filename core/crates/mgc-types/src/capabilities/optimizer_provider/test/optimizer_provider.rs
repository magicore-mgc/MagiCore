//! Optimizer provider capability contract tests.
//! Test hợp đồng capability optimizer provider.

use crate::capabilities::OptimizerProvider;
use crate::capabilities::test_support::{MockCore, assert_unsupported};

#[test]
fn default_probe_fails_closed_with_original_contract() {
    assert_unsupported(
        MockCore.probe_optimizer().unwrap_err(),
        "web",
        "optimizer",
        "this core has no optimizer/bench capability",
    );
}
