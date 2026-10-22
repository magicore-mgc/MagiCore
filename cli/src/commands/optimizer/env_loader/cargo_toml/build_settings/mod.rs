//! Cargo `[build]` setting validation.
//! (Kiểm tra setting `[build]` của Cargo.)

mod build_settings;

pub(super) use build_settings::parse_build_settings;

#[cfg(test)]
#[path = "test/build_settings.rs"]
mod tests;
