#![cfg(test)]

//! Board queries preserve framework-specific ordering and reject cross-framework targets.
//! Truy vấn board giữ thứ tự theo framework và từ chối target chéo framework.

use crate::framework::{board_target, board_target_for_framework, boards_for_framework};

#[test]
fn board_choices_keep_registry_order_and_alias_behavior() {
    let ids = |framework: &str| {
        boards_for_framework(framework)
            .into_iter()
            .map(|board| board.id)
            .collect::<Vec<_>>()
    };
    assert_eq!(ids("esp32-rust"), ["esp32c3", "esp32s3", "esp32"]);
    assert_eq!(ids("platformio"), ["esp32dev", "nodemcu-32s"]);
    assert_eq!(ids("zephyr-arm"), ["nrf52dk_nrf52832", "stm32f4_disc"]);
    assert!(ids("unknown-framework").is_empty());
}

#[test]
fn target_lookup_preserves_legacy_and_framework_scoped_routes() {
    assert_eq!(
        board_target("esp32c3").as_deref(),
        Some("riscv32imc-unknown-none-elf")
    );
    assert_eq!(
        board_target_for_framework("esp32-rust", "esp32c3").as_deref(),
        Some("riscv32imc-unknown-none-elf")
    );
    assert_eq!(
        board_target_for_framework("esp32-rust", "nrf52dk_nrf52832"),
        None
    );
}
