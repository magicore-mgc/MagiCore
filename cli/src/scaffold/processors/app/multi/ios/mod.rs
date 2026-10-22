//! iOS and Objective-C bridge output wiring.
//! Wiring output iOS và cầu nối Objective-C.

#[path = "ios.rs"]
mod implementation;
pub(in crate::scaffold::processors::app::implementation) use implementation::write;
