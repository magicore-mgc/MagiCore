//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn app_rn_unsupported_flutter_delegated_unknown_unsupported() {
    // P0#2: the app/rn rule fires through the real gate path.
    // Luật app/rn được kiểm tra qua đúng đường gate runtime.
    for op in [
        DepOp::Install,
        DepOp::Add,
        DepOp::Remove,
        DepOp::Update,
        DepOp::List,
    ] {
        for mode in [native(), explicit("flutter"), explicit("npm")] {
            let err = gate(&ctx("app", Some(eco::RN), op), None, &mode, None).unwrap_err();
            let message = err.to_string();
            assert!(
                message.contains("disabled for dependency operations") || message.contains("rn"),
                "failure must explain the compat ban or unsupported RN lane: {err}"
            );
        }
    }
    assert!(
        gate(
            &ctx("app", Some(eco::FLUTTER), DepOp::Install),
            Some("flutter"),
            &native(),
            None
        )
        .is_ok()
    );
    // Native mode accepts Flutter install; this test does not request compat.
    // Flutter install được chấp nhận ở native mode; test không bật compat.
    assert!(
        gate(
            &ctx("app", Some(eco::FLUTTER), DepOp::Install),
            Some("flutter"),
            &native(),
            None
        )
        .is_ok(),
        "flutter install is native (no toolchain spawn)"
    );
    assert!(
        gate(
            &ctx("app", Some(eco::FLUTTER), DepOp::Add),
            None,
            &native(),
            None
        )
        .is_ok()
    );
    // Undetected app ecosystem never falls back to generic-delegated.
    // Không detect được ecosystem App thì không fallback sang delegated chung.
    assert!(gate(&ctx("app", None, DepOp::Install), None, &native(), None).is_err());
    assert!(gate(&ctx("app", None, DepOp::Install), None, &native(), None).is_err());
}
