//! Core identity contract contract tests.
//! Test hợp đồng hợp đồng định danh core.

use crate::Ecosystem;
use crate::capabilities::CoreIdent;
use crate::capabilities::test_support::MockCore;

#[test]
fn identity_exposes_stable_core_name_and_ecosystem() {
    assert_eq!(MockCore.core_id(), "web");
    assert_eq!(MockCore.name(), "Web");
    assert_eq!(MockCore.ecosystem(), Ecosystem::Web);
}
