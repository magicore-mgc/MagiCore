//! java artifact-fetch owner wiring — wiring owner fetch artifact java.

#[path = "java.rs"]
mod implementation;

pub(in crate::adapter) use implementation::fetch;
