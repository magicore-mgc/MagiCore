//! Prepared Add owner exports — xuất dữ liệu chuẩn bị thêm package.

mod prepared_add;
pub use prepared_add::PreparedAdd;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
