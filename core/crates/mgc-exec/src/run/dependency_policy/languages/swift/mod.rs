//! Swift Package Manager policy facade.
//! Facade policy Swift Package Manager.

#[path = "swift.rs"]
mod implementation;
pub(super) use implementation::denial_reason;
