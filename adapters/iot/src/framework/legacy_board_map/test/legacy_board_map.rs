#![cfg(test)]

//! Verify source-compatible board-map entries stay available.
//! Xác minh entry map board tương thích source vẫn còn.

use crate::framework::KNOWN_BOARDS;

#[test]
fn legacy_snapshot_keeps_all_seven_board_triples() {
    assert_eq!(KNOWN_BOARDS.len(), 7);
    assert!(KNOWN_BOARDS.contains(&("esp32c3", "esp32c3", "riscv32imac-unknown-none-elf")));
}
