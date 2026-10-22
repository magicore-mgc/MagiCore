//! Simulator provider capability contract tests.
//! Test hợp đồng capability simulator provider.

use crate::capabilities::SimulatorProvider;
use crate::capabilities::test_support::{MockCore, assert_unsupported};

#[test]
fn default_probe_fails_closed_with_original_contract() {
    assert_unsupported(
        MockCore.probe_simulator().unwrap_err(),
        "web",
        "simulate",
        "this core has no simulator capability",
    );
}
