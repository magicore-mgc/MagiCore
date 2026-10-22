//! Stable project-core marker APIs and ownership checks.
//! API marker core ổn định và kiểm tra ownership.
use std::path::Path;

use super::super::ProjectConfig;
use super::{
    error::{CoreMarkerError, Result as CoreMarkerResult},
    identity, reader, writer,
};

impl ProjectConfig {
    // Plain-text core identity marker file; this is not a cryptographic signature.
    // File identity core dạng text thường; đây không phải chữ ký mật mã.
    pub const CORE_MARKER_FILE: &str = ".mgc.core";

    /// Core names accepted by the marker; `cloud` canonicalizes to `clo`.
    /// Các tên core được marker chấp nhận; `cloud` chuẩn hóa thành `clo`.
    pub const KNOWN_CORES: &[&str] = &[
        "web", "game", "ai", "clo", "cicd", "iot", "app", "lib", "hardware",
    ];

    /// Canonicalize a core name (trim, lowercase, alias mapping).
    /// Chuẩn hóa tên core (trim, chữ thường và ánh xạ bí danh).
    pub(crate) fn canonical_core(name: &str) -> String {
        identity::canonical_core(name)
    }

    pub(crate) fn is_known_core(name: &str) -> bool {
        identity::is_known_core(name)
    }

    /// Read core marker from project root and enforce its attestation policy.
    /// Đọc marker core và áp dụng policy attestation hiện hành.
    pub fn read_core_marker(project_root: &Path) -> Result<Option<String>, anyhow::Error> {
        into_anyhow(reader::read_core_marker(project_root))
    }

    /// Parse and cross-check identity without enforcing attestation.
    /// Đọc và đối chiếu identity mà không enforce attestation.
    pub(crate) fn read_core_marker_identity(
        project_root: &Path,
    ) -> Result<Option<String>, anyhow::Error> {
        into_anyhow(identity::read_core_marker_identity(project_root))
    }

    /// Write the marker for this configuration's core.
    /// Ghi marker cho core của cấu hình này.
    pub fn write_core_marker(&self, project_root: &Path) -> Result<(), anyhow::Error> {
        into_anyhow(writer::write_core_marker(project_root, &self.ecosystem))
    }

    /// Claim an absent marker or verify an existing marker has the same core.
    /// Nhận ownership nếu marker chưa có; marker khác core luôn bị từ chối.
    pub fn ensure_core_marker_at(project_root: &Path, core: &str) -> Result<(), anyhow::Error> {
        into_anyhow(writer::ensure_core_marker_at(project_root, core))
    }

    /// Write a core marker only when it agrees with any existing project config.
    /// Chỉ ghi marker khi nhất quán với cấu hình project hiện có.
    pub fn write_core_marker_at(project_root: &Path, core: &str) -> Result<(), anyhow::Error> {
        into_anyhow(writer::write_core_marker_at(project_root, core))
    }
}

fn into_anyhow<T>(result: CoreMarkerResult<T>) -> Result<T, anyhow::Error> {
    result.map_err(CoreMarkerError::into_anyhow)
}
