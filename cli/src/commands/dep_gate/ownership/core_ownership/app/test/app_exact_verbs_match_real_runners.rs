//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn app_exact_verbs_match_real_runners() {
    // Native operations pass; source/catalog mutations without transaction
    // support and every compat mode fail.
    // Operation native thì chạy; lane chưa hỗ trợ và mọi compat đều bị chặn.
    assert!(
        gate(
            &ctx("app", Some(eco::FLUTTER), DepOp::Update),
            Some("flutter"),
            &native(),
            None
        )
        .is_ok()
    );
    assert!(
        gate(
            &ctx("app", Some(eco::SWIFT), DepOp::Install),
            None,
            &native(),
            None
        )
        .is_ok()
    );
    for op in [DepOp::Add, DepOp::Remove, DepOp::Update] {
        assert!(gate(&ctx("app", Some(eco::SWIFT), op), None, &native(), None).is_err());
        assert!(gate(&ctx("app", Some(eco::KOTLIN), op), None, &native(), None).is_err());
    }
    assert!(
        gate(
            &ctx("app", Some(eco::SWIFT), DepOp::List),
            None,
            &native(),
            None
        )
        .is_err()
    );
    assert!(
        gate(
            &ctx("app", Some(eco::KOTLIN), DepOp::Install),
            None,
            &native(),
            None
        )
        .is_err()
    );
    assert!(
        gate(
            &ctx("app", Some(eco::KOTLIN), DepOp::List),
            None,
            &native(),
            None
        )
        .is_err()
    );
    assert!(
        gate(
            &ctx("app", Some(eco::OBJC), DepOp::Install),
            None,
            &native(),
            None
        )
        .is_err()
    );
    for core_eco_op in [
        ("app", eco::FLUTTER, DepOp::Install),
        ("app", eco::SWIFT, DepOp::Install),
    ] {
        assert!(
            gate(
                &ctx(core_eco_op.0, Some(core_eco_op.1), core_eco_op.2),
                None,
                &explicit("flutter"),
                None
            )
            .is_err()
        );
    }
}
