//! Local encoding error wiring — Wiring lỗi encoding local.
//! Re-export the private taxonomy and register display tests — Re-export taxonomy private và đăng ký test display.

mod error;
pub(super) use error::EncodingError;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
