//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn non_native_iot_package_operations_are_unsupported_even_with_compat() {
    for (framework, tool) in [("zephyr", "west"), ("platformio", "pio")] {
        for op in [DepOp::Install, DepOp::Add, DepOp::Remove, DepOp::Update] {
            assert!(
                gate(
                    &ctx("iot", Some(framework), op),
                    Some(tool),
                    &explicit(tool),
                    None
                )
                .is_err(),
                "iot/{framework} {op:?} must remain unsupported until MGC owns the lifecycle"
            );
        }
    }
}
