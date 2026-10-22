//! Safe Unix config-directory owner.
//! Owner tạo chuỗi thư mục cấu hình an toàn.

mod ensure_directory_chain;

pub(super) use ensure_directory_chain::ensure_directory_chain;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
