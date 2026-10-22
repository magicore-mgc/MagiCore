//! Wires owns the typed refusal when legacy data cannot safely become v4.
//! Nối owner: Sở hữu lỗi từ chối có kiểu khi dữ liệu cũ không thể đổi an toàn sang v4.

mod error;
pub use self::error::LossyMigration;
#[cfg(test)]
mod test;
