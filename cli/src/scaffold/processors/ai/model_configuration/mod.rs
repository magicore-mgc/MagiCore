//! Model Configuration output owner wiring.
//! Wiring owner ghi output model configuration.

#[path = "model_configuration.rs"]
mod implementation;
pub(super) use implementation::write;
