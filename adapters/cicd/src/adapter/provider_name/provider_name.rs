//! Expose the configured provider's canonical display string.
//! Cung cấp chuỗi hiển thị canonical của provider đã cấu hình.

use super::super::adapter::CicdAdapter;

impl CicdAdapter {
    pub fn provider(&self) -> &'static str {
        self.provider.as_str()
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
