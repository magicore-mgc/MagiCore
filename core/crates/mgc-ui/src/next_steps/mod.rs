//! Next-steps wiring — wiring cho hướng dẫn bước tiếp.
//! Re-exports the existing root function and registers its nearest test.
//! Re-export hàm root và đăng ký test gần owner.

mod next_steps;
#[cfg(test)]
#[path = "test/next_steps.rs"]
mod tests;

pub use next_steps::print_next_steps;
