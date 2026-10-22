//! Podfile.lock parser wiring.
//! Nối parser Podfile.lock.

#[path = "parse.rs"]
mod implementation;

pub use implementation::parse_podfile_lock;
