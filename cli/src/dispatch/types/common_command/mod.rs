//! CommonCommand owner exports — stores shared CLI command payloads and options.
//! Module xuất payload và tùy chọn của các lệnh CLI dùng chung.
mod common_command;
pub use common_command::CommonCommand;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
