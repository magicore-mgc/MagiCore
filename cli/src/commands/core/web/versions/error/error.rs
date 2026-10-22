//! Typed diagnostics for Web version and persisted release policy failures.
//! Diagnostic có kiểu cho lỗi version Web và release policy đã lưu.

use anyhow::Error;
use std::path::{Path, PathBuf};

/// Local error taxonomy retaining stable diagnostics across the compatibility facade.
/// Taxonomy lỗi tại chỗ giữ nguyên diagnostic qua facade tương thích.
#[derive(Debug, thiserror::Error)]
pub(crate) enum WebVersionError {
    #[error(
        "cannot enforce minimum-release-age for scaffold package '{package}': registry metadata is unavailable: {cause}"
    )]
    AgeMetadataUnavailable { package: String, cause: String },
    #[error(
        "cannot resolve the current registry version for scaffold package '{package}': {cause}; retry when registry access is available"
    )]
    RegistryVersionUnavailable { package: String, cause: String },
    #[error("cannot load web scaffold security policy from '{project_root}': {cause}", project_root = .project_root.display())]
    SecurityConfigUnreadable {
        project_root: PathBuf,
        cause: String,
    },
    #[error("cannot read web scaffold release policy from '{path}': {cause}", path = .path.display())]
    StorePolicyUnreadable { path: PathBuf, cause: String },
    #[error("web scaffold release policy database '{path}' must be a regular file", path = .path.display())]
    StorePolicyPathInvalid { path: PathBuf },
    #[error("npm metadata for '{package}' has no latest dist-tag")]
    LatestTagMissing { package: String },
    #[error("npm latest dist-tag for '{package}' is invalid")]
    LatestTagInvalid { package: String },
    #[error("npm metadata for '{package}' has no '{tag}' dist-tag")]
    DistTagMissing { package: String, tag: String },
    #[error("npm '{tag}' dist-tag for '{package}' is invalid")]
    DistTagInvalid { package: String, tag: String },
    #[error(
        "npm '{tag}' dist-tag for '{package}' points to {version}, which does not satisfy the configured minimum-release-age policy"
    )]
    DistTagNotEligible {
        package: String,
        tag: String,
        version: mgc_types::Version,
    },
    #[error("no stable release of '{package}' satisfies the configured minimum-release-age policy")]
    NoEligibleRelease { package: String },
    #[error(
        "no stable release of '{package}' in scaffold range '{range}' satisfies the configured minimum-release-age policy"
    )]
    NoEligibleReleaseInRange { package: String, range: String },
}

/// Build the existing age-gated metadata diagnostic at the local owner boundary.
/// Dựng diagnostic metadata age gate cũ tại ranh giới owner cục bộ.
pub fn web_scaffold_age_metadata_unavailable(
    package: &str,
    cause: &dyn std::fmt::Display,
) -> Error {
    Error::new(WebVersionError::AgeMetadataUnavailable {
        package: package.to_string(),
        cause: cause.to_string(),
    })
}

/// Build the current-registry diagnostic without supplying a historical fallback.
/// Tạo lỗi tra version hiện hành mà không thay bằng bản lịch sử ghim sẵn.
pub fn web_scaffold_registry_version_unavailable(
    package: &str,
    cause: &dyn std::fmt::Display,
) -> Error {
    Error::new(WebVersionError::RegistryVersionUnavailable {
        package: package.to_string(),
        cause: cause.to_string(),
    })
}

/// Build the project-security configuration diagnostic.
/// Tạo diagnostic khi không đọc được cấu hình security của project.
pub fn web_scaffold_security_config_unreadable(
    project_root: &Path,
    cause: &dyn std::fmt::Display,
) -> Error {
    Error::new(WebVersionError::SecurityConfigUnreadable {
        project_root: project_root.to_path_buf(),
        cause: cause.to_string(),
    })
}

/// Build the persisted release-policy read diagnostic.
/// Tạo diagnostic khi không đọc được release policy đã lưu.
pub fn web_scaffold_store_policy_unreadable(path: &Path, cause: &dyn std::fmt::Display) -> Error {
    Error::new(WebVersionError::StorePolicyUnreadable {
        path: path.to_path_buf(),
        cause: cause.to_string(),
    })
}

/// Reject a persisted policy path that is not a regular file.
/// Từ chối đường dẫn policy đã lưu nếu nó không phải file thường.
pub fn web_scaffold_store_policy_path_invalid(path: &Path) -> Error {
    Error::new(WebVersionError::StorePolicyPathInvalid {
        path: path.to_path_buf(),
    })
}

/// Report npm metadata that lacks its latest tag.
/// Báo lỗi metadata npm không có latest tag.
pub fn web_scaffold_latest_tag_missing(package: &str) -> Error {
    Error::new(WebVersionError::LatestTagMissing {
        package: package.to_string(),
    })
}

/// Report an invalid npm latest tag.
/// Báo lỗi latest tag npm không hợp lệ.
pub fn web_scaffold_latest_tag_invalid(package: &str) -> Error {
    Error::new(WebVersionError::LatestTagInvalid {
        package: package.to_string(),
    })
}

/// Report npm metadata that lacks the requested tag.
/// Báo lỗi metadata npm thiếu tag được yêu cầu.
pub fn web_scaffold_dist_tag_missing(package: &str, tag: &str) -> Error {
    Error::new(WebVersionError::DistTagMissing {
        package: package.to_string(),
        tag: tag.to_string(),
    })
}

/// Report a requested npm tag that does not identify a valid version.
/// Báo lỗi tag npm được yêu cầu không trỏ tới version hợp lệ.
pub fn web_scaffold_dist_tag_invalid(package: &str, tag: &str) -> Error {
    Error::new(WebVersionError::DistTagInvalid {
        package: package.to_string(),
        tag: tag.to_string(),
    })
}

/// Reject a requested tag whose release is too new under the active policy.
/// Từ chối release của tag còn quá mới theo policy hiện hành.
pub fn web_scaffold_dist_tag_not_eligible(
    package: &str,
    tag: &str,
    version: &mgc_types::Version,
) -> Error {
    Error::new(WebVersionError::DistTagNotEligible {
        package: package.to_string(),
        tag: tag.to_string(),
        version: version.clone(),
    })
}

/// Report that no stable release passed the active age policy.
/// Báo không có stable release nào qua được age policy hiện hành.
pub fn web_scaffold_no_eligible_release(package: &str) -> Error {
    Error::new(WebVersionError::NoEligibleRelease {
        package: package.to_string(),
    })
}

/// Report that no release in the requested range passed the active age policy.
/// Báo range yêu cầu không có release nào qua được age policy hiện hành.
pub fn web_scaffold_no_eligible_release_in_range(package: &str, range: &str) -> Error {
    Error::new(WebVersionError::NoEligibleReleaseInRange {
        package: package.to_string(),
        range: range.to_string(),
    })
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
