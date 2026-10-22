//! Reload and updated-path notifications.
//! Thông báo reload và path vừa cập nhật.

/// Event sent to connected development clients.
/// Event gửi tới client đang kết nối dev server.
#[derive(Debug, Clone)]
pub enum HmrEvent {
    Reload,
    /// Path of the changed file — đường dẫn file vừa thay đổi.
    Update(String),
}

#[cfg(test)]
#[path = "test/event.rs"]
mod tests;
