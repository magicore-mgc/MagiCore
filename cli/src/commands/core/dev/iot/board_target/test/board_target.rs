use super::*;

#[test]
fn chip_resolves_from_registry() {
    assert_eq!(chip("esp32c3"), "esp32c3");
    assert_eq!(chip("unknown-board"), "unknown-board");
}

#[test]
fn flash_target_override_cannot_bypass_board_registry() {
    assert_eq!(
        resolve_target("esp32-rust", "esp32c3", None).unwrap(),
        "riscv32imac-unknown-none-elf"
    );
    assert_eq!(
        resolve_target("esp32-rust", "esp32c3", Some("custom-target")).unwrap(),
        "custom-target"
    );
    assert!(resolve_target("esp32-rust", "unknown-board", Some("custom-target")).is_err());
    assert!(resolve_target("esp32-rust", "nrf52dk_nrf52832", Some("custom-target")).is_err());
}
