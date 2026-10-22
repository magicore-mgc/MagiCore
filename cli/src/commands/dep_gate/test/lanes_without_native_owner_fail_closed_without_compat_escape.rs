//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn lanes_without_native_owner_fail_closed_without_compat_escape() {
    for (core, eco, op) in [
        ("ai", Some(eco::PYTHON), DepOp::Remove),
        ("clo", Some(eco::TERRAFORM), DepOp::Install),
    ] {
        let err = gate(&ctx(core, eco, op), None, &native(), None).unwrap_err();
        assert!(
            err.to_string().contains("unsupported") || err.to_string().contains("does not yet own"),
            "{core} {} must fail without a compat escape: {err}",
            op.as_str()
        );
    }
}
