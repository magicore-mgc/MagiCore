//! Keyring module wiring and public exports — Wiring module và export keyring.
//! Keep the crate's established public type paths — Giữ public type path hiện hành.

mod error;
mod key_pair;
mod keyring;
mod storage;

pub use key_pair::KeyPair;
pub use keyring::Keyring;

#[cfg(test)]
#[path = "test/error.rs"]
mod error_tests;
