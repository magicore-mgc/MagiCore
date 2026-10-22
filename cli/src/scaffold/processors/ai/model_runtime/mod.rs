//! Model Runtime output owner wiring.
//! Wiring owner ghi output model runtime.

#[path = "model_runtime.rs"]
mod implementation;
pub(super) use implementation::write;
