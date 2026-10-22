//! Define the configuration-time patch kind and package specification.
//! Định nghĩa loại patch và spec package ở thời điểm cấu hình.

use crate::package::VersionRange;
use serde::{Deserialize, Serialize};

/// Patch kind: P1 `Diff` uses the internal apply engine; P2 `Prebuilt` is a binary.
/// Loại patch: P1 `Diff` dùng engine nội bộ; P2 `Prebuilt` là binary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PatchKind {
    #[default]
    Diff,
    Prebuilt,
}

/// Store package patch specs in `mgc.toml` under `[patches]`; current format is
/// npm-only for web/lib TypeScript.
/// Lưu spec patch trong `mgc.toml` tại `[patches]`; định dạng hiện tại chỉ dùng
/// npm cho TypeScript của web/lib.
/// A6: `mgc.lock` patch fields are npm-format only; other cores use standard lockfiles.
/// A6: trường patch của `mgc.lock` chỉ dùng npm-format; core khác dùng lockfile chuẩn.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchSpec {
    /// Package name, for example `react` or `@scope/pkg`.
    /// Tên package, ví dụ `react` hoặc `@scope/pkg`.
    pub package: String,
    /// Version range the patch applies to, for example `1.0.0 - 1.2.0`.
    /// Khoảng phiên bản được áp dụng patch, ví dụ `1.0.0 - 1.2.0`.
    pub version_range: VersionRange,
    /// Project-relative patch file path under `.magicore/patches/`.
    /// Đường dẫn file patch tương đối trong `.magicore/patches/` của dự án.
    pub patch_path: String,
    /// SHA-256 digest of the diff content used for integrity verification.
    /// Digest SHA-256 của nội dung diff dùng để xác minh toàn vẹn.
    pub integrity: String,
    /// Patch representation; defaults to `Diff`.
    /// Dạng patch; mặc định là `Diff`.
    #[serde(default)]
    pub kind: PatchKind,
}

impl PatchSpec {
    /// Construct a patch spec using the existing default kind.
    /// Tạo spec patch với kind mặc định như hiện tại.
    pub fn new(
        package: String,
        version_range: VersionRange,
        patch_path: String,
        integrity: String,
    ) -> Self {
        Self {
            package,
            version_range,
            patch_path,
            integrity,
            kind: PatchKind::Diff,
        }
    }
}
