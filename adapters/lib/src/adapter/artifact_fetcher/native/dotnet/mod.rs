//! dotnet artifact-fetch owner wiring — wiring owner fetch artifact dotnet.

#[path = "dotnet.rs"]
mod implementation;

pub(in crate::adapter) use implementation::fetch;
