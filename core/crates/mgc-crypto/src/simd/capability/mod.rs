//! SIMD capability owner wiring — Kết nối module sở hữu capability SIMD.
//! Export the public value and register its focused tests — Export value public và đăng ký test chuyên biệt.

mod capability;
pub use capability::SimdCapability;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
