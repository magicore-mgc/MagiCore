//! Password prompt wiring — wiring prompt password.
//! Re-exports the existing password input function.
//! Re-export hàm nhập password hiện tại.

mod password;
#[cfg(test)]
#[path = "test/password.rs"]
mod tests;

pub use password::password;
