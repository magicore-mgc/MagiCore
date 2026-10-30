//! Cargo source readiness: marker plus verified checksum record.
//! Sẵn sàng source Cargo: marker và checksum record đã xác minh.
//!
//! Both artifacts exist for different readers and neither replaces the other:
//! `cargo` itself needs `.cargo-ok` to accept a `registry/src` directory as
//! complete, while MagiCore's own build gate and later installs need
//! `.cargo-checksum.json` to re-verify the tree against the locked archive.
//! Writing only one of them produces the failure this owner exists to remove:
//! Cargo re-extracts a directory without its marker, deleting the checksum
//! record, and the next build reports that `.cargo-checksum.json` is missing.
//! Hai artifact phục vụ hai bên đọc khác nhau và không thay thế nhau: `cargo`
//! cần `.cargo-ok` để coi thư mục `registry/src` là hoàn tất, còn cổng build và
//! lần install sau của MagiCore cần `.cargo-checksum.json` để xác minh lại cây
//! với archive đã khóa. Chỉ ghi một trong hai sẽ tạo đúng lỗi mà owner này sinh
//! ra để loại bỏ: Cargo giải nén lại thư mục thiếu marker, xoá checksum record,
//! và lần build sau báo thiếu `.cargo-checksum.json`.

use std::path::{Path, PathBuf};

use super::super::error::RustNativeInstallError;
use crate::install::cargo_compat;

/// Locate `registry/src/{name}-{version}` under a Cargo-layout home.
/// Xác định `registry/src/{name}-{version}` dưới một home layout Cargo.
pub(crate) fn source_root(cargo_home: &Path, name: &str, version: &str) -> PathBuf {
    mgc_resolver::protocols::registry_source_root(cargo_home)
        .join(mgc_resolver::protocols::registry_package_dir(name, version))
}

/// Finish an extracted registry source so Cargo and MagiCore both accept it.
///
/// Order matters: the checksum record is computed from the archive and verified
/// against the extracted files first, then Cargo's marker is written as the very
/// last step. The in-memory readiness probe is a cheap invariant check for later
/// edits, not the source of truth.
/// Hoàn tất source registry đã giải nén để cả Cargo và MagiCore chấp nhận. Thứ
/// tự quan trọng: checksum record được tính từ archive và xác minh với file đã
/// giải nén trước, marker của Cargo được ghi ở bước cuối cùng. Probe trong bộ nhớ
/// chỉ là kiểm invariant rẻ cho các lần sửa sau, không phải nguồn chân lý.
pub(crate) fn finalize_source(
    package_root: &Path,
    package_sha256: &str,
    archive_bytes: &[u8],
) -> Result<(), RustNativeInstallError> {
    cargo_compat::write_source_checksum(package_root, package_sha256, archive_bytes)?;
    mgc_resolver::protocols::write_cargo_source_marker(package_root)?;
    debug_assert!(mgc_resolver::protocols::has_cargo_source_marker(
        package_root
    ));
    Ok(())
}

/// Reject a source directory that Cargo would silently re-extract.
///
/// A directory without the marker is an interrupted materialization: its
/// contents may be partial, and Cargo will delete them on the next build. The
/// caller rebuilds such a source instead of trusting it.
/// Từ chối thư mục source mà Cargo sẽ âm thầm giải nén lại. Thiếu marker nghĩa là
/// materialize bị ngắt: nội dung có thể dở dang và Cargo sẽ xoá nó ở lần build
/// sau. Caller phải dựng lại source đó thay vì tin nó.
pub(crate) fn require_cargo_source_marker(
    package_root: &Path,
) -> Result<(), RustNativeInstallError> {
    if mgc_resolver::protocols::has_cargo_source_marker(package_root) {
        return Ok(());
    }
    Err(RustNativeInstallError::SourceNotMaterialized {
        path: package_root.display().to_string(),
    })
}

/// Marker write failure keeps the install error's filesystem origin.
/// Lỗi ghi marker giữ nguyên nguồn gốc filesystem của lỗi install.
impl From<std::io::Error> for RustNativeInstallError {
    fn from(source: std::io::Error) -> Self {
        Self::SourceMarker { source }
    }
}
