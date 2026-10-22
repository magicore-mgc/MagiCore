//! Project trust-root settings.
//! Cấu hình trust root của project.

#[path = "trust.rs"]
mod trust;

pub use trust::TrustConfig;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
