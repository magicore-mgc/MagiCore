//! `.env` key/value parser and nearest tests.
//! (Parser key/value `.env` và kiểm thử gần owner.)

mod env_file;

pub(crate) use env_file::parse_env_file;

#[cfg(test)]
#[path = "test/env_file.rs"]
mod tests;
