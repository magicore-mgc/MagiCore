//! Lockfile provider capability contract tests.
//! Test hợp đồng capability lockfile provider.

use crate::capabilities::LockfileProvider;
use crate::capabilities::test_support::{MockCore, assert_unsupported};

#[test]
fn default_probe_fails_closed_with_original_contract() {
    assert_unsupported(
        MockCore.probe_lockfile_provider().unwrap_err(),
        "web",
        "write_manifest",
        "this core does not use mgc-written manifests; regenerate via `mgc create-<core>` or edit the manifest directly",
    );
}
