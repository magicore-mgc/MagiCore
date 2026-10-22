#![cfg(test)]

//! Keep the board choice fields available to callers.
//! Giữ các field lựa chọn board sẵn có cho caller.

use crate::framework::BoardInfo;

#[test]
fn board_info_fields_retain_the_public_shape() {
    let board = BoardInfo {
        id: "esp32c3".into(),
        label: "ESP32-C3".into(),
        chip: "esp32c3".into(),
        target: "riscv32imac-unknown-none-elf".into(),
    };
    assert_eq!(board.id, "esp32c3");
    assert_eq!(board.target, "riscv32imac-unknown-none-elf");
}
