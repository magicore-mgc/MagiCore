//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn ai_python_dependency_gate_rejects_compat_for_every_operation() {
    for op in [
        DepOp::Install,
        DepOp::Add,
        DepOp::Remove,
        DepOp::Update,
        DepOp::List,
    ] {
        assert!(
            gate(&ctx("ai", Some(eco::PYTHON), op), None, &native(), None).is_err(),
            "AI Python {op:?} without an owned manifest must fail closed"
        );
        let owned = DepContext::new("ai", Some(eco::PYTHON), None, None, op)
            .with_manifest_format(Some("mgc-pyproject"));
        assert!(gate(&owned, None, &native(), None).is_ok());
        assert!(
            gate(
                &ctx("ai", Some(eco::PYTHON), op),
                Some("uv"),
                &explicit("uv"),
                None
            )
            .is_err()
        );
    }
    // Undetected ai ecosystem never defaults open.
    // Không detect được ecosystem AI thì không mở mặc định.
    assert!(gate(&ctx("ai", None, DepOp::Install), None, &native(), None).is_err());
}
