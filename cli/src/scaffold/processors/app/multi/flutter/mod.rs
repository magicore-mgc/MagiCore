//! Flutter output wiring.
//! Wiring output Flutter.

#[path = "flutter.rs"]
mod implementation;
pub(in crate::scaffold::processors::app::implementation) use implementation::write;
