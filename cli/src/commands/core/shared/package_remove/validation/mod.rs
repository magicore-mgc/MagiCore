//! Remove request validation wiring.
//! Wiring kiểm tra request remove.

#[path = "validation.rs"]
mod implementation;

pub(crate) use implementation::validate_remove_request;
