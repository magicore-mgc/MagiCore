//! Multi-platform context owner wiring.
//! Wiring owner của context đa nền tảng.

#[path = "context.rs"]
mod implementation;
pub(in crate::scaffold::processors::app::implementation) use implementation::MultiProjectContext;
