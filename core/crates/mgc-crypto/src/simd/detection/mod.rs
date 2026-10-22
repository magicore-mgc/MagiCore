//! SIMD detection owner wiring — Wiring owner phát hiện SIMD.
//! Register architecture-sensitive tests beside detection — Đăng ký test theo kiến trúc cạnh module detect.

mod detection;
pub use detection::detect_simd;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
