//! Device provider capability contract tests.
//! Test hợp đồng capability device provider.

use crate::capabilities::DeviceProvider;
use crate::capabilities::test_support::{MockCore, assert_unsupported};

#[test]
fn default_probe_fails_closed_with_original_contract() {
    assert_unsupported(
        MockCore.probe_device().unwrap_err(),
        "web",
        "device",
        "this core has no device capability",
    );
}
