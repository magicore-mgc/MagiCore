//! Command-name presentation — trình bày tên lệnh.
//! Styling remains presentation-only and does not validate command capability.
//! Định dạng chỉ phục vụ hiển thị, không xác nhận capability của lệnh.

use console::style;

/// Style a command name for display — định dạng tên lệnh để hiển thị.
/// The returned string is presentation only — chuỗi trả về chỉ phục vụ hiển thị.
pub fn style_cmd(cmd: &str) -> String {
    format!("{}", style(cmd).cyan().bold())
}
