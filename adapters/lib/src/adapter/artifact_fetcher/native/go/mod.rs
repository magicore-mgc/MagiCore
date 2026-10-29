//! go artifact-fetch owner wiring — wiring owner fetch artifact go.

#[path = "go.rs"]
mod implementation;

pub(in crate::adapter) use implementation::fetch;
