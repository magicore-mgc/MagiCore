//! Agent output owner wiring.
//! Wiring owner ghi output agent.

#[path = "agent.rs"]
mod implementation;
pub(super) use implementation::write;
