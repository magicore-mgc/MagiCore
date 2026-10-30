//! Repair Cargo-managed registry sources that are missing Cargo's readiness marker.
//! Vá source registry do Cargo quản lý khi thiếu marker sẵn sàng của Cargo.
//!
//! Older MagiCore releases wrote `.cargo-checksum.json` but not Cargo's own
//! `.cargo-ok` marker. Cargo treats such a directory as absent, re-extracts it,
//! and deletes the checksum record, so the build then fails with
//! "failed to load checksum `.cargo-checksum.json` ... No such file or directory"
//! (reported for `icu_segmenter v2.4.0`). Replacing the whole source set from the
//! verified `.crate` cache would be correct but needs a full reinstall on every
//! existing store, so this owner repairs the marker in place — only after the
//! checksum record has been re-verified against the cached archive and the
//! extracted tree, which keeps the same trust bar as `mgc install`.
//! Bản MagiCore cũ ghi `.cargo-checksum.json` nhưng không ghi marker `.cargo-ok`
//! của Cargo. Cargo coi thư mục đó như không tồn tại, giải nén lại và xoá
//! checksum record, nên build lỗi "failed to load checksum `.cargo-checksum.json`
//! ... No such file or directory" (đã báo cho `icu_segmenter v2.4.0`). Thay toàn
//! bộ source từ cache `.crate` đã xác minh là đúng nhưng buộc reinstall lại trên
//! mọi store hiện có, nên owner này vá marker tại chỗ — chỉ sau khi checksum
//! record được xác minh lại với archive đã cache và cây đã giải nén, giữ đúng mức
//! tin cậy như `mgc install`.

use std::path::Path;

use anyhow::Result;
use mgc_ui::info;

/// Repair every package directory a managed build is about to hand to Cargo.
///
/// Returns how many directories were repaired so the caller can log a single
/// line instead of one per crate.
/// Vá mọi thư mục package mà build managed sắp giao cho Cargo. Trả số thư mục đã
/// vá để caller log một dòng thay vì mỗi crate một dòng.
pub(crate) fn repair_managed_sources(source_root: &Path) -> Result<usize> {
    let mut repaired = 0usize;
    let entries = std::fs::read_dir(source_root).map_err(|error| {
        anyhow::anyhow!(
            "cannot read managed Cargo source '{}': {error}; run `mgc install`",
            source_root.display()
        )
    })?;
    for entry in entries {
        let entry = entry?;
        let package_root = entry.path();
        let metadata = std::fs::symlink_metadata(&package_root)?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            continue;
        }
        let package_dir = entry.file_name().to_string_lossy().into_owned();
        let Some((name, version)) = split_package_dir(&package_dir) else {
            continue;
        };
        let archive_path = source_root
            .parent()
            .map(|registry| registry.join("cache").join(format!("{package_dir}.crate")))
            .ok_or_else(|| anyhow::anyhow!("Cargo source registry path has no parent"))?;
        let archive = read_archive(&archive_path)?;
        match repair_package(&package_root, name, version, &archive)? {
            RepairOutcome::Ready => {}
            RepairOutcome::Repaired => repaired += 1,
        }
    }
    if repaired > 0 {
        info(&format!(
            "Repaired Cargo source readiness marker for {repaired} managed package(s)"
        ));
    }
    Ok(repaired)
}

/// What happened to one package directory.
/// Kết quả xử lý một thư mục package.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum RepairOutcome {
    /// The directory already carried a valid marker.
    /// Thư mục đã có marker hợp lệ.
    Ready,
    /// The marker was missing and has been written.
    /// Marker thiếu và đã được ghi.
    Repaired,
}

/// Decide whether a package directory needs the marker, and repair it.
///
/// A directory with no `.cargo-checksum.json` is left untouched: it was not
/// materialized by MagiCore (a hand-placed directory must not gain a marker that
/// claims MagiCore vetted it), and the build gate already refuses it.
/// Quyết định thư mục package có cần marker và vá nó. Thư mục không có
/// `.cargo-checksum.json` được để nguyên: nó không do MagiCore materialize
/// (không được gắn marker khẳng định MagiCore đã xác minh), và cổng build đã
/// từ chối nó.
pub(crate) fn repair_package(
    package_root: &Path,
    name: &str,
    version: &str,
    archive: &[u8],
) -> Result<RepairOutcome> {
    if mgc_resolver::protocols::has_cargo_source_marker(package_root) {
        return Ok(RepairOutcome::Ready);
    }
    let checksum_path = package_root.join(".cargo-checksum.json");
    if !checksum_path.is_file() {
        return Ok(RepairOutcome::Ready);
    }
    let expected_sha256 = archive_digest(archive);
    mgc_lib_adapter::install::cargo_compat::verify_managed_source(
        package_root,
        &expected_sha256,
        archive,
    )
    .map_err(|error| {
        anyhow::anyhow!(
            "managed Cargo source for '{name}'@{version} failed re-verification: {error}; \
                 run `mgc install` to rebuild it"
        )
    })?;
    mgc_resolver::protocols::write_cargo_source_marker(package_root)?;
    Ok(RepairOutcome::Repaired)
}

/// SHA-256 of a cached archive; the directory source must match this digest.
/// SHA-256 của archive đã cache; directory source phải khớp digest này.
fn archive_digest(archive: &[u8]) -> String {
    use sha2::Digest as _;
    hex::encode(sha2::Sha256::digest(archive))
}

/// Split Cargo's `{name}-{version}` directory name at the last dash before a
/// version that starts with a digit (crate names may contain dashes).
/// Tách tên thư mục `{name}-{version}` của Cargo tại dấu gạch cuối trước version
/// bắt đầu bằng chữ số (tên crate có thể chứa gạch).
pub(crate) fn split_package_dir(package_dir: &str) -> Option<(&str, &str)> {
    package_dir
        .rmatch_indices('-')
        .find(|(index, _)| {
            package_dir[index + 1..]
                .chars()
                .next()
                .is_some_and(|first| first.is_ascii_digit())
        })
        .map(|(index, _)| (&package_dir[..index], &package_dir[index + 1..]))
}

/// Read the cached `.crate` archive a source directory was materialized from.
/// Đọc archive `.crate` đã cache mà thư mục source được materialize từ đó.
fn read_archive(archive_path: &Path) -> Result<Vec<u8>> {
    std::fs::read(archive_path).map_err(|error| {
        anyhow::anyhow!(
            "cannot read managed Cargo archive '{}': {error}; run `mgc install`",
            archive_path.display()
        )
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
