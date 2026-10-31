//! Select validated board choices and preserve compatibility lookups.
//! Chọn board đã validate và giữ các hàm tra cứu tương thích.

use crate::framework::{BoardInfo, board_registry::board_records, canonical_framework};

/// Return board choices that the selected framework can actually scaffold.
/// Trả các board mà framework đã chọn thật sự có thể scaffold.
/// Unknown framework identifiers return an empty list.
/// Framework không biết sẽ trả danh sách rỗng.
pub fn boards_for_framework(framework: &str) -> Vec<BoardInfo> {
    let Some(framework) = canonical_framework(framework) else {
        return Vec::new();
    };
    let mut boards = board_records()
        .iter()
        .filter(|board| board.frameworks.iter().any(|item| item == framework))
        .collect::<Vec<_>>();
    boards.sort_by_key(|board| board.wizard_order);
    boards
        .into_iter()
        .map(|board| BoardInfo {
            id: board.id.clone(),
            label: board.label.clone(),
            chip: board.chip.clone(),
            target: board.target.clone(),
        })
        .collect()
}

pub fn known_boards() -> Vec<(String, String, String)> {
    board_records()
        .iter()
        .map(|board| (board.id.clone(), board.chip.clone(), board.target.clone()))
        .collect()
}

pub fn board_target(board: &str) -> Option<String> {
    board_records()
        .iter()
        .find(|entry| entry.id == board)
        .map(|entry| entry.target.clone())
}

/// Resolve a target only when the board belongs to the selected framework.
/// Chỉ lấy target nếu board thuộc framework đã chọn.
pub fn board_target_for_framework(framework: &str, board: &str) -> Option<String> {
    boards_for_framework(framework)
        .into_iter()
        .find(|entry| entry.id == board)
        .map(|entry| entry.target)
}

/// Resolve the chip name (the `esp-hal` feature id) only when the board
/// belongs to the selected framework.
/// Chỉ lấy tên chip (id feature `esp-hal`) nếu board thuộc framework đã chọn.
pub fn board_chip_for_framework(framework: &str, board: &str) -> Option<String> {
    boards_for_framework(framework)
        .into_iter()
        .find(|entry| entry.id == board)
        .map(|entry| entry.chip)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
