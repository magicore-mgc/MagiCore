//! Build the Swift registry-sourced Package.resolved pin.
//! Tạo pin Package.resolved có nguồn registry Swift.

use mgc_resolver::protocols::swift::SwiftResolvedPin;
use mgc_types::Version;

/// Preserve the current slash-to-dot and lowercase registry identity projection.
/// Giữ phép chiếu identity registry hiện tại: slash thành chấm và chữ thường.
pub(in crate::install::swift_native) fn build_registry_pin(
    name: &str,
    version: &Version,
) -> SwiftResolvedPin {
    SwiftResolvedPin {
        identity: name.replace('/', ".").to_lowercase(),
        version: Some(version.to_string()),
        revision: None,
        branch: None,
        location: None,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
