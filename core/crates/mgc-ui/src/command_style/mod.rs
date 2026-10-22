//! Command-name styling wiring — wiring cho định dạng tên lệnh.
//! Re-exports the existing root helper without adding a new public module.
//! Re-export helper gốc mà không thêm public module mới.

mod command_style;
#[cfg(test)]
#[path = "test/command_style.rs"]
mod tests;

pub use command_style::style_cmd;
