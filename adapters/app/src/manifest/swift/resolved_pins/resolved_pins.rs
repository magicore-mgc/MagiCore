//! Reads SwiftPM pins and maps them to registry or Git requirements.
//! Đọc pin SwiftPM và ánh xạ thành requirement registry hoặc Git.

use mgc_resolver::protocols::swift::SwiftDep;
use mgc_types::{MgError, MgResult};
use std::path::Path;

use mgc_adapter_base::project_file::read_regular_text;
use mgc_resolver::protocols::swift::{SwiftResolvedPin, parse_package_resolved};

/// Read Package.resolved pins (absent file = no pins, not an error).
/// Đọc pin Package.resolved (thiếu file = không pin, không phải lỗi).
pub(in crate::manifest::swift) fn read_pins(
    project_root: &Path,
) -> MgResult<Vec<SwiftResolvedPin>> {
    let path = project_root.join("Package.resolved");
    match std::fs::symlink_metadata(&path) {
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => {
            return Err(MgError::Other(format!("inspect Package.resolved: {error}")));
        }
    }
    let text = read_regular_text(&path, "Package.resolved")?;
    parse_package_resolved(&text)
}

/// Match a registry pin by identity or a remote pin by repository identity/location.
/// Khớp pin registry theo identity hoặc pin remote theo identity/location repository.
pub(in crate::manifest::swift) fn pin_range_for(
    dep: &SwiftDep,
    pins: &[SwiftResolvedPin],
) -> Option<String> {
    match dep {
        SwiftDep::Registry { identity, .. } => {
            let normalized = identity.to_lowercase();
            pins.iter()
                .find(|pin| pin.identity.to_lowercase() == normalized)
                .and_then(pin_requirement)
        }
        SwiftDep::Git { url, .. } => {
            let repo = url
                .trim_end_matches('/')
                .rsplit('/')
                .next()
                .unwrap_or_default()
                .trim_end_matches(".git")
                .to_lowercase();
            pins.iter()
                .find(|pin| {
                    pin.identity.to_lowercase() == repo
                        || pin.location.as_ref().is_some_and(|location| {
                            location
                                .trim_end_matches(".git")
                                .ends_with(&url_to_tail(url))
                        })
                })
                .and_then(pin_requirement)
        }
    }
}

/// Tail of a Git URL (`host/path/repo`) used for pin location matching.
/// Đuôi URL Git (`host/path/repo`) dùng để khớp location pin.
fn url_to_tail(url: &str) -> String {
    url.trim_end_matches('/')
        .trim_end_matches(".git")
        .rsplit("://")
        .next()
        .unwrap_or(url)
        .trim_end_matches(".git")
        .to_string()
}

/// Convert a resolved pin to the requirement prefix understood by the resolver.
/// Chuyển pin đã resolve sang prefix requirement mà resolver hiểu.
fn pin_requirement(pin: &SwiftResolvedPin) -> Option<String> {
    let is_remote = pin
        .location
        .as_ref()
        .is_some_and(|location| location.contains("://") && !location.starts_with("registry+"));
    if let Some(version) = &pin.version {
        return Some(if is_remote {
            format!("tag:{version}")
        } else {
            format!("exact:{version}")
        });
    }
    if let Some(revision) = &pin.revision {
        return Some(format!("revision:{revision}"));
    }
    if let Some(branch) = &pin.branch {
        return Some(format!("branch:{branch}"));
    }
    None
}
