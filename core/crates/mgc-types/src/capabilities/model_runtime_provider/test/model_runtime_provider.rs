//! Model runtime capability contract tests.
//! Test hợp đồng capability runtime model.

use crate::capabilities::ModelRuntimeProvider;
use crate::capabilities::test_support::{MockCore, assert_unsupported};

#[test]
fn default_probe_fails_closed_with_original_contract() {
    assert_unsupported(
        MockCore.probe_model_runtime().unwrap_err(),
        "web",
        "model-runtime",
        "this core has no model runtime capability",
    );
}
