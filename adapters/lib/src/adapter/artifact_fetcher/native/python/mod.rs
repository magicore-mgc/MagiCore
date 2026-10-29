//! python artifact-fetch owner wiring — wiring owner fetch artifact python.

#[path = "python.rs"]
mod implementation;

pub(in crate::adapter) use implementation::fetch;
