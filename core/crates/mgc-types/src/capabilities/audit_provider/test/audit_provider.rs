//! Audit provider capability contract tests.
//! Test hợp đồng capability audit provider.

use crate::capabilities::AuditProvider;
use crate::capabilities::test_support::{MockCore, assert_unsupported};

#[test]
fn default_probe_fails_closed_with_original_contract() {
    assert_unsupported(
        MockCore.probe_audit_provider().unwrap_err(),
        "web",
        "audit",
        "no scanner implemented for this core",
    );
}
