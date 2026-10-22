//! Cargo package/registry and offline-build policy facade.
//! Facade policy package/registry và build offline của Cargo.

#[path = "cargo.rs"]
mod implementation;
pub(super) use implementation::denial_reason;
