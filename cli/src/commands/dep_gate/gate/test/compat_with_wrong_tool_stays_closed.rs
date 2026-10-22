//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn compat_with_wrong_tool_stays_closed() {
    for (core, ecosystem, operation, tool) in [
        ("ai", Some(eco::PYTHON), DepOp::Install, "cargo"),
        ("ai", Some(eco::PYTHON), DepOp::Remove, "pip"),
        ("clo", Some(eco::TERRAFORM), DepOp::Install, "terraform"),
    ] {
        let error = gate(
            &ctx(core, ecosystem, operation),
            Some(tool),
            &explicit(tool),
            None,
        )
        .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("disabled for dependency operations"),
            "{error}"
        );
    }
}
