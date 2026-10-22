//! Cargo `[env]` allowlist and reserved-key checks.
//! (Allowlist `[env]` Cargo và kiểm tra key bị cấm ghi đè.)

mod env_settings;

pub(super) use env_settings::parse_env_settings;

#[cfg(test)]
#[path = "test/env_settings.rs"]
mod tests;
