//! Materializer capability contract tests.
//! Test hợp đồng capability materializer.

use crate::capabilities::Materializer;
use crate::capabilities::test_support::{MockCore, assert_unsupported};

#[test]
fn default_probe_fails_closed_with_original_contract() {
    assert_unsupported(
        MockCore.probe_materializer().unwrap_err(),
        "web",
        "materialize",
        "this core does not materialize a dependency tree layout",
    );
}
