//! Android output wiring.
//! Wiring output Android.

#[path = "android.rs"]
mod implementation;
pub(in crate::scaffold::processors::app::implementation) use implementation::write;
