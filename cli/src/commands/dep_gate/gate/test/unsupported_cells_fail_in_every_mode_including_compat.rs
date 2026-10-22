//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn unsupported_cells_fail_in_every_mode_including_compat() {
    for mode in [native(), explicit("cargo"), explicit("terraform")] {
        for (core, eco, op) in [
            ("cicd", None, DepOp::Install),
            ("hardware", None, DepOp::Add),
            ("unknown-core", None, DepOp::Install),
            ("app", Some(eco::RN), DepOp::Install),
            ("lib", None, DepOp::Install),
            // Exact cells without runners (never Delegated promises).
            // Các cell dưới đây không có runner, không được báo là delegated.
            ("app", Some(eco::SWIFT), DepOp::Add),
            ("app", Some(eco::OBJC), DepOp::List),
            ("app", Some(eco::OBJC), DepOp::Add),
            // Terraform has no native dependency lifecycle for these cells.
            // Terraform chưa có dependency lifecycle native cho các cell này.
            ("clo", Some(eco::TERRAFORM), DepOp::Add),
            ("clo", Some(eco::TERRAFORM), DepOp::Remove),
            // Non-bevy game engines have no package manager.
            // Engine game ngoài Bevy không có package manager của MGC.
            ("game", Some("godot"), DepOp::Install),
            ("game", Some("unity"), DepOp::Add),
        ] {
            assert!(
                gate(&ctx(core, eco, op), None, &mode, None).is_err(),
                "{core}/{eco:?} {} must fail closed even under compat",
                op.as_str()
            );
        }
    }
}
