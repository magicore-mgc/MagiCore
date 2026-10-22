//! Shared TOML string serialization boundary.
//! Ranh giới serialize chuỗi TOML dùng chung.

mod toml_string;
pub(in crate::scaffold) use toml_string::serialize_toml_string;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
