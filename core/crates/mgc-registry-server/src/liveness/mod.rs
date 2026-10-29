//! Liveness probe owner wiring — wiring owner probe sống.

#[path = "liveness.rs"]
mod implementation;

pub(crate) use implementation::routes;
