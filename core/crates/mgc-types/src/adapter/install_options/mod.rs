//! Install Options owner exports — xuất kiểu tùy chọn cài đặt.

mod install_options;
pub use install_options::InstallOptions;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
