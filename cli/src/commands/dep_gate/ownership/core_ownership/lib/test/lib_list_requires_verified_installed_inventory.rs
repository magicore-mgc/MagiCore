//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn lib_list_requires_verified_installed_inventory() {
    for eco in [Some(eco::PYTHON), Some(eco::TS)] {
        assert!(gate(&ctx("lib", eco, DepOp::List), None, &native(), None).is_ok());
    }
    for eco in [
        None,
        Some(eco::RUST),
        Some(eco::GO),
        Some(eco::JAVA),
        Some(eco::DOTNET),
        Some("unknown-lang"),
    ] {
        assert!(gate(&ctx("lib", eco, DepOp::List), None, &native(), None).is_err());
    }
}
