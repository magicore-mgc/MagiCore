//! Python package and build policy facade.
//! Facade policy package và build Python.

#[path = "python.rs"]
mod implementation;
pub(super) use implementation::denial_reason;
