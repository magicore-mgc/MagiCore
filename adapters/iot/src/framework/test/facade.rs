#![cfg(test)]

//! Keep the published IoT framework and board APIs importable.
//! Giữ API framework và board IoT đã công bố có thể import.

use crate::framework::{
    BoardInfo, IotFramework, KNOWN_BOARDS, board_target, board_target_for_framework,
    boards_for_framework, detect_framework, known_boards,
};

#[test]
fn stable_iot_framework_exports_remain_available() {
    let _: Option<IotFramework> = IotFramework::from_str("esp32-rust");
    let _: Option<BoardInfo> = boards_for_framework("esp32-rust").into_iter().next();
    let _ = KNOWN_BOARDS;
    let _ = board_target("esp32");
    let _ = board_target_for_framework("esp32-rust", "esp32");
    let _ = detect_framework;
    let _ = known_boards;
}
