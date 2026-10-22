//! Verification owner wiring — Wiring owner xác minh.
//! Re-export the stable operation and register nearest tests — Re-export operation ổn định và đăng ký test gần owner.

mod verification;

pub use verification::verify_adler32;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
