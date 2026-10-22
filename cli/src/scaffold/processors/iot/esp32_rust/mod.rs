//! ESP-HAL Rust writer owner wiring.
//! Wiring owner writer rust esp-hal.

#[path = "esp32_rust.rs"]
mod implementation;
pub(super) use implementation::write;
