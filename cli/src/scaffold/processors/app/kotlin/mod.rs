//! Kotlin owner wiring.
//! Wiring owner Kotlin.

#[path = "kotlin.rs"]
mod implementation;
pub(super) use implementation::write;
