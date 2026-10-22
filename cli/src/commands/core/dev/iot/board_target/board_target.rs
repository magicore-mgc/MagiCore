use anyhow::Result;

/// Validate board ownership before honoring an explicit target override.
/// Xác thực board trước khi nhận target override tường minh.
pub(in crate::commands::core::dev) fn resolve_target(
    framework: &str,
    board: &str,
    target_override: Option<&str>,
) -> Result<String> {
    let registered =
        mgc_iot_adapter::board_target_for_framework(framework, board).ok_or_else(|| {
            let boards = mgc_iot_adapter::boards_for_framework(framework)
                .iter()
                .map(|entry| entry.id.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            crate::error::unsupported_board(board, &boards)
        })?;
    Ok(target_override.unwrap_or(&registered).to_string())
}

/// Board id → chip (tra registry KNOWN_BOARDS).
pub(in crate::commands::core::dev) fn chip(board: &str) -> String {
    mgc_iot_adapter::boards_for_framework("esp32-rust")
        .iter()
        .find(|entry| entry.id == board)
        .map(|entry| entry.chip.clone())
        .unwrap_or_else(|| board.to_string())
}
