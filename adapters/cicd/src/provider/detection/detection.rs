//! CI/CD provider detection precedence.
//! Thứ tự ưu tiên nhận diện provider CI/CD.

use std::path::Path;

use crate::provider::CicdProvider;

use super::explicit_config::ProviderConfigOutcome;

/// Detect a configured provider or apply the existing marker order.
/// Nhận diện provider từ config hoặc áp dụng thứ tự marker hiện hữu.
pub fn detect_provider(root: &Path) -> Option<CicdProvider> {
    match super::explicit_config::read_configured_provider(root) {
        ProviderConfigOutcome::Recognized(provider) => Some(provider),
        ProviderConfigOutcome::UnsupportedName => None,
        ProviderConfigOutcome::NotConfigured => super::markers::provider_from_markers(root),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
