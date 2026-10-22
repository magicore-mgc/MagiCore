//! Content store capability contract tests.
//! Test hợp đồng capability content store.

use crate::capabilities::ContentStoreProvider;
use crate::capabilities::test_support::{MockCore, assert_unsupported};

#[test]
fn default_probe_fails_closed_with_original_contract() {
    assert_unsupported(
        MockCore.probe_content_store().unwrap_err(),
        "web",
        "install",
        "this core has no mgc registry install; dependencies are materialized by the core's own workflow",
    );
}
