//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn web_is_native_only_for_declared_js_ts() {
    for eco in [Some(eco::JS), Some(eco::TS)] {
        for op in [
            DepOp::Install,
            DepOp::Add,
            DepOp::Remove,
            DepOp::Update,
            DepOp::List,
            DepOp::Resolve,
            DepOp::Gc,
        ] {
            assert!(gate(&ctx("web", eco, op), None, &native(), None).is_ok());
            assert!(gate(&ctx("web", eco, op), None, &explicit("cargo"), None).is_err());
        }
    }
    // Undeclared ecosystem never defaults open.
    // Ecosystem không khai báo không được mở mặc định.
    assert!(gate(&ctx("web", None, DepOp::Install), None, &native(), None).is_err());
    assert!(
        gate(
            &ctx("web", Some("python"), DepOp::Install),
            None,
            &native(),
            None
        )
        .is_err()
    );
}
