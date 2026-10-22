//! SIMD info owner wiring — Wiring owner thông tin SIMD.
//! Export the established summary function and register its test — Export hàm summary cũ và đăng ký test.

mod info;
pub use info::simd_info;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
