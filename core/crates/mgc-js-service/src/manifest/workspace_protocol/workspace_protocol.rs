//! Recognize workspace dependency range syntax.
//! Nhận diện cú pháp range dependency workspace.

/// Return whether a version range uses the `workspace:` protocol.
/// Cho biết version range có dùng protocol `workspace:` hay không.
pub fn is_workspace_protocol_range(range: &str) -> bool {
    let range = range.trim();
    range.starts_with("workspace:")
        || range == "workspace:*"
        || range == "workspace:^"
        || range == "workspace:~"
}
