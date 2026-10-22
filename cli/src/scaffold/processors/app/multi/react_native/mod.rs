//! React Native output wiring.
//! Wiring output React Native.

#[path = "react_native.rs"]
mod implementation;
pub(in crate::scaffold::processors::app::implementation) use implementation::write;
