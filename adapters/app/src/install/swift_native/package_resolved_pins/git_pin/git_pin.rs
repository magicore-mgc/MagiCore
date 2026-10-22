//! Build the Swift Git-sourced Package.resolved pin projection.
//! Tạo projection pin Package.resolved có nguồn Git Swift.

use mgc_resolver::protocols::swift::SwiftResolvedPin;
use mgc_types::Version;

/// Preserve tag-shaped version state, lock revision provenance and location formatting.
/// Giữ version dạng tag, revision provenance từ lock và format location.
pub(in crate::install::swift_native) fn build_git_pin(
    name: &str,
    version: &Version,
    revision: Option<String>,
) -> SwiftResolvedPin {
    SwiftResolvedPin {
        identity: name.rsplit('/').next().unwrap_or(name).to_lowercase(),
        version: Version::parse(&version.to_string())
            .ok()
            .map(|_| version.to_string()),
        revision,
        branch: None,
        location: Some(format!("https://{name}.git")),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
