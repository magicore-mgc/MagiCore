//! Web scaffold flag exports.
//! Xuất cờ scaffold dành riêng cho Web.
mod scaffold_flags;

pub use scaffold_flags::ScaffoldFlags;
/// Explicit name for the Web-only scaffold flag type.
/// Tên tường minh cho kiểu cờ scaffold chỉ dành cho Web.
pub type WebScaffoldFlags = ScaffoldFlags;
