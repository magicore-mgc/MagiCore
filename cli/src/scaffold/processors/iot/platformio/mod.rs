//! PlatformIO writer owner wiring.
//! Wiring owner writer platformio.

#[path = "platformio.rs"]
mod implementation;
pub(super) use implementation::write;
