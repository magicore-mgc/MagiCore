#![cfg(test)]

//! Loading the embedded board registry validates a non-empty canonical set.
//! Nạp registry board nhúng xác nhận tập canonical không rỗng.

use crate::framework::board_registry::board_records;

#[test]
fn embedded_registry_contains_validated_boards() {
    assert!(!board_records().is_empty());
}
