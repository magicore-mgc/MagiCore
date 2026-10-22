//! Google Cloud Build output owner wiring.
//! Wiring owner output Google Cloud Build.

#[path = "gcp.rs"]
mod implementation;
pub(super) use implementation::write;
