//! Add request validation wiring.
//! Wiring kiểm tra request add.

#[path = "validation.rs"]
mod implementation;

pub(crate) use implementation::validate_add_request;
