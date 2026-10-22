//! Multi-platform App owner wiring.
//! Wiring owner App đa nền tảng.

#[path = "context/mod.rs"]
mod context;
pub(super) use context::MultiProjectContext;
#[path = "multi.rs"]
mod implementation;
pub(super) use implementation::write;
