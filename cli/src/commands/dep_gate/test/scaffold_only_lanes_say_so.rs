//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn scaffold_only_lanes_say_so() {
    // P1: cicd/hardware failures must read as scaffold-only, never as a
    // package install that "isn't supported yet".
    // Lỗi CI/CD/Hardware phải nói scaffold-only, không phải package install chưa hỗ trợ.
    for (core, op) in [("cicd", DepOp::Install), ("hardware", DepOp::Add)] {
        let err = gate(&ctx(core, None, op), None, &native(), None).unwrap_err();
        assert!(
            err.to_string().contains("scaffold-only"),
            "{core} must be labeled scaffold-only: {err}"
        );
    }
    // Hardware list is a REAL read-only inventory command: it passes the
    // gate, but its owner is ScaffoldOnly — never "mgc-native".
    // Hardware list là inventory chỉ đọc thật; owner là ScaffoldOnly, không phải native.
    assert!(gate(&ctx("hardware", None, DepOp::List), None, &native(), None).is_ok());
    assert!(matches!(
        owner_for(&ctx("hardware", None, DepOp::List)),
        DepOwner::ScaffoldOnly
    ));
}
