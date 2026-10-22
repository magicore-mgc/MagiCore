//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn game_and_iot_list_are_blocked_without_installed_state_evidence() {
    for (core, ecosystem, framework) in [
        ("game", Some(eco::BEVY), Some(eco::BEVY)),
        ("iot", Some("esp32-rust"), Some("esp32-rust")),
        ("iot", Some("platformio"), Some("platformio")),
    ] {
        let context = DepContext::new(core, ecosystem, framework, None, DepOp::List);
        assert!(matches!(owner_for(&context), DepOwner::Unsupported));
    }
}
