//! Cargo profile field translation and validation.
//! (Chuyển đổi và kiểm tra field Cargo profile.)

mod profile_settings;

pub(super) use profile_settings::parse_profile_settings;

#[cfg(test)]
#[path = "test/profile_settings.rs"]
mod tests;
