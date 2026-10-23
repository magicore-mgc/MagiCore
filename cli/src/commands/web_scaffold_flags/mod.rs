//! Web scaffold option schema owner (core-independent clap data).
//! Owner schema tùy chọn scaffold Web (dữ liệu clap độc lập core).

mod web_scaffold_flags;

pub use web_scaffold_flags::ScaffoldFlags;

/// Explicit name for the Web-only scaffold flag type.
/// Tên tường minh cho kiểu cờ scaffold chỉ dành cho Web.
pub type WebScaffoldFlags = ScaffoldFlags;
