//! Board Registry behavior tests.
//! Test hành vi cho nhóm board registry.

use mgc_iot_adapter::{
    KNOWN_BOARDS, board_target, board_target_for_framework, boards_for_framework, known_boards,
};

pub fn embedded_board_registry_compatibility_api_matches_snapshot() {
    let compatibility_snapshot = KNOWN_BOARDS
        .iter()
        .map(|(id, chip, target)| (id.to_string(), chip.to_string(), target.to_string()))
        .collect::<Vec<_>>();

    assert_eq!(known_boards(), compatibility_snapshot);
    for (id, _, target) in compatibility_snapshot {
        assert_eq!(board_target(&id).as_deref(), Some(target.as_str()));
    }
}

pub fn board_registry_filters_choices_by_framework_and_rejects_cross_framework_targets() {
    let ids = |framework: &str| {
        boards_for_framework(framework)
            .into_iter()
            .map(|board| board.id)
            .collect::<Vec<_>>()
    };

    assert_eq!(ids("esp32-rust"), ["esp32c3", "esp32s3", "esp32"]);
    assert_eq!(ids("platformio"), ["esp32dev", "nodemcu-32s"]);
    assert_eq!(ids("zephyr"), ["nrf52dk_nrf52832", "stm32f4_disc"]);
    assert_eq!(ids("zephyr-arm"), ids("zephyr"));
    assert!(ids("unknown-framework").is_empty());

    assert_eq!(
        board_target_for_framework("esp32-rust", "esp32c3").as_deref(),
        Some("riscv32imac-unknown-none-elf")
    );
    assert_eq!(
        board_target_for_framework("esp32-rust", "nrf52dk_nrf52832"),
        None
    );
    assert_eq!(
        board_target_for_framework("unknown-framework", "esp32c3"),
        None
    );
}

pub fn platformio_esp32dev_registry_maps_to_its_esp32_target() {
    let board = boards_for_framework("platformio")
        .into_iter()
        .find(|board| board.id == "esp32dev")
        .expect("esp32dev is a registered PlatformIO board");

    assert_eq!(board.chip, "esp32");
    assert_eq!(board.target, "xtensa-esp32-none-elf");
}
