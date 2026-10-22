//! Web package unlink command facade.
//! Facade lệnh gỡ package link Web.

#[path = "unlink.rs"]
mod implementation;

pub use implementation::unlink;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
