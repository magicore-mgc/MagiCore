//! rust artifact-fetch owner wiring — wiring owner fetch artifact rust.

#[path = "rust.rs"]
mod implementation;

pub(in crate::adapter) use implementation::fetch;
