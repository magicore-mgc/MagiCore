//! Rust compiler policy facade.
//! Facade policy compiler Rust.

#[path = "rustc.rs"]
mod implementation;
pub(super) use implementation::denial_reason;
