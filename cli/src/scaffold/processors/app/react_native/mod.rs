//! React Native owner wiring.
//! Wiring owner React Native.

#[path = "react_native.rs"]
mod implementation;
pub(super) use implementation::write;
