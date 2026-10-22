//! Install Summary owner exports — xuất kết quả và chế độ cache cài đặt.

mod install_summary;
pub use install_summary::{InstallCacheMode, InstallSummary};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
