//! Forwarded-flag owner wiring — wiring owner kiểm flag chuyển tiếp.

#[path = "forwarded_flags.rs"]
mod implementation;

pub use implementation::validate_forwarded_runtime_flags;
