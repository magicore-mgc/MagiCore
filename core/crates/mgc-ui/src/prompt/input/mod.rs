//! Text-input prompt wiring — wiring prompt nhập văn bản.
//! Re-exports the stable input function.
//! Re-export hàm input ổn định.

mod input;
#[cfg(test)]
#[path = "test/input.rs"]
mod tests;

pub use input::input;
