//! Confirm prompt wiring — wiring prompt xác nhận.
//! Exposes the existing confirm function through prompt.
//! Expose hàm confirm hiện tại qua module prompt.

mod confirm;
#[cfg(test)]
#[path = "test/confirm.rs"]
mod tests;

pub use confirm::confirm;
