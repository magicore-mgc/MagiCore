//! Digest type owner wiring — Wiring owner kiểu digest.
//! Export the stable value and register its nearest tests — Export value ổn định, đăng ký test gần owner.

mod types;
pub use types::Blake3Hash;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
