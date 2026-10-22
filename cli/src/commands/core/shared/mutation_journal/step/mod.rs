//! Journaled step module wiring.
//! Wiring module bước mutation có journal.

#[path = "step.rs"]
mod implementation;

pub(crate) use implementation::journaled_step;
