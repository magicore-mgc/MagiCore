//! ESP-HAL error owner wiring.
//! Wiring owner lỗi ESP-HAL.

#[path = "error.rs"]
mod implementation;
pub(super) use implementation::IotEsp32Error;
