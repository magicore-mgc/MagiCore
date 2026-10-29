//! Registry request parser with bounded names and selectors.
//! Parser request registry có giới hạn tên và selector.

use anyhow::Result;

const MAX_REGISTRY_PACKAGE_REQUEST_BYTES: usize = 4096;
/// Local alias of the shared npm limit; the value lives in `mgc_types::limits`
/// so this validator and the registry server cannot disagree.
/// (Alias cục bộ của giới hạn npm dùng chung.)
const MAX_NPM_PACKAGE_NAME_BYTES: usize = mgc_types::limits::NPM_PACKAGE_NAME_MAX;

pub(crate) fn parse_request(request: &str) -> Result<(&str, &str)> {
    if request.is_empty() || request.len() > MAX_REGISTRY_PACKAGE_REQUEST_BYTES {
        return Err(super::registry_package_invalid_request(
            "empty or oversized request",
        ));
    }
    let (name, selector) = match request.rfind('@') {
        Some(position) if position > 0 => (&request[..position], &request[position + 1..]),
        _ => (request, "latest"),
    };
    let valid_segment = |segment: &str| {
        !segment.is_empty()
            && !segment.starts_with(['.', '_'])
            && segment
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    };
    let valid_name = if let Some(scoped) = name.strip_prefix('@') {
        scoped
            .split_once('/')
            .is_some_and(|(scope, package)| valid_segment(scope) && valid_segment(package))
    } else {
        valid_segment(name)
    };
    if name.len() > MAX_NPM_PACKAGE_NAME_BYTES || !valid_name {
        return Err(super::registry_package_invalid_request(
            "invalid registry package name",
        ));
    }
    if selector.trim().is_empty()
        || !selector.is_ascii()
        || selector.bytes().any(|byte| {
            byte.is_ascii_control() || matches!(byte, b'/' | b'\\' | b':' | b'%' | b'#' | b'?')
        })
    {
        return Err(super::registry_package_invalid_request(
            "expected a registry version, range, or tag; paths and URLs are unsupported",
        ));
    }
    Ok((name, selector))
}
#[cfg(test)]
#[path = "test/request_validation.rs"]
mod tests;
