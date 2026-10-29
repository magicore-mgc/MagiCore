//! Versioned credential protection and bounded password work.
//! Bảo vệ credential có phiên bản và giới hạn tác vụ mật khẩu.

mod constant_time;
mod error;
mod password;
mod token;
mod user;
mod worker;

pub(crate) use constant_time::constant_time_eq;
pub(crate) use error::CredentialError;
pub(crate) use password::{hash_password, valid_password_hash, verify_password_hash};
pub(crate) use token::TokenDigest;
pub(crate) use user::ProtectedUser;
pub(crate) use worker::run_kdf;

mod configuration_debug;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
