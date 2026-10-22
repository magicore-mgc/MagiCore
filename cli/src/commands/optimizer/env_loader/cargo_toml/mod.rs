//! Validated Cargo optimizer TOML pipeline.
//! (Luồng parse và kiểm tra TOML Cargo optimizer.)

mod build_settings;
mod cargo_toml;
mod env_settings;
mod profile_settings;

pub(super) use cargo_toml::parse_toml_file;

#[cfg(test)]
#[path = "test/cargo_toml.rs"]
mod tests;
