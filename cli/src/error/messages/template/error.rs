//! Template fetch, integrity, and publication diagnostics.
//! Lỗi tải, toàn vẹn và publish template.

use anyhow::Error;
use anyhow::anyhow;
use std::fmt;

/// Typed marker for transport failures so template cache policy can classify retries.
/// Marker có kiểu cho lỗi transport để policy cache template phân loại retry.
#[derive(Debug)]
pub(crate) struct TemplateTransportUnavailable;

impl fmt::Display for TemplateTransportUnavailable {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("template transport is unavailable")
    }
}

impl std::error::Error for TemplateTransportUnavailable {}

/// Typed HTTP status preserved for template cache retry decisions.
/// Giữ status HTTP có kiểu để quyết định retry cache template.
#[derive(Debug)]
pub(crate) struct TemplateHttpStatus(pub(crate) u16);

impl fmt::Display for TemplateHttpStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "template registry returned HTTP {}", self.0)
    }
}

impl std::error::Error for TemplateHttpStatus {}

/// templates/web missing on disk
pub fn web_templates_missing(root: &std::path::Path) -> Error {
    anyhow!("templates/web does not exist on disk: {}", root.display())
}

/// no web layer found with template.toml+sources
pub fn no_web_layer_found(dir: &std::path::Path) -> Error {
    anyhow!(
        "No web layer with template.toml+sources found in {}",
        dir.display()
    )
}

/// template publish required before fetch
pub fn template_not_published(url: &str, http: u16) -> Error {
    anyhow!(
        "Template '{}' not found at {} (HTTP {}) — publish it first (`mgc publish --help`)",
        url,
        url,
        http
    )
}

pub fn template_download_url_invalid() -> Error {
    anyhow!(
        "template downloads require credential-free HTTPS URLs, or HTTP on a loopback registry without HTTPS downgrade"
    )
}

pub fn template_request_failed() -> Error {
    anyhow!("template registry request failed or exceeded its timeout")
}

pub fn template_redirect_invalid() -> Error {
    anyhow!("template registry returned an invalid redirect or exceeded the redirect limit")
}

pub fn template_transport_unavailable() -> Error {
    anyhow!(TemplateTransportUnavailable)
}

pub fn template_download_status(status: u16) -> Error {
    anyhow!(TemplateHttpStatus(status))
}

pub fn template_download_too_large(limit: usize) -> Error {
    anyhow!("template response exceeds the {limit} byte limit")
}

pub fn template_integrity_invalid() -> Error {
    anyhow!("template archive requires a valid SHA-256, SHA-384, or SHA-512 SRI digest")
}

pub fn template_integrity_mismatch() -> Error {
    anyhow!("template archive integrity mismatch")
}

pub fn template_archive_layout_invalid() -> Error {
    anyhow!("template archive must contain one package directory with template.toml and sources")
}

pub fn template_cache_rollback_failed(
    target: &std::path::Path,
    retained: &std::path::Path,
    publish: &dyn std::fmt::Display,
    rollback: &dyn std::fmt::Display,
) -> Error {
    anyhow!(
        "cannot publish template cache '{}': {publish}; rollback failed: {rollback}; previous cache retained under '{}'",
        target.display(),
        retained.display()
    )
}

/// Invalid manifest source/target path — đường dẫn template không được thoát root.
/// Bound manifest parsing allocation — giới hạn bộ nhớ trước parse manifest.
/// Publishing errors never echo registry bodies or credential-bearing URLs.
/// Không đưa response body hoặc URL có credential vào lỗi publish.
pub fn template_publish_status(status: u16) -> Error {
    if status == 409 {
        anyhow!("Template version already exists; bump --version")
    } else {
        anyhow!("Template publish failed with HTTP {}", status)
    }
}

pub fn template_publish_auth_required() -> Error {
    anyhow!("Template registry requires authentication; set MGC_NPM_TOKEN")
}
