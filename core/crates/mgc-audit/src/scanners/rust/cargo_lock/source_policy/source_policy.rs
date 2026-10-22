//! Keep accepted crates.io source spellings and skip text in one owner.
//! Tập trung cách viết source crates.io được nhận và nội dung skip.

const CRATES_IO_GIT_INDEX: &str = "registry+https://github.com/rust-lang/crates.io-index";
const CRATES_IO_SPARSE_INDEX: &str = "sparse+https://index.crates.io/";
const CRATES_IO_REGISTRY_INDEX: &str = "registry+https://index.crates.io/";

/// Return whether a source is one of the existing crates.io registry forms.
/// Cho biết source có thuộc một trong các dạng registry crates.io hiện có không.
pub(in crate::scanners::rust::cargo_lock) fn is_crates_io_registry(source: &str) -> bool {
    matches!(
        source,
        CRATES_IO_GIT_INDEX | CRATES_IO_SPARSE_INDEX | CRATES_IO_REGISTRY_INDEX
    )
}

/// Preserve the existing reason text for an unsupported explicit source.
/// Giữ nguyên nội dung lý do cho explicit source không hỗ trợ.
pub(in crate::scanners::rust::cargo_lock) fn skipped_reason(
    name: &str,
    version: &str,
    source: &str,
) -> String {
    format!("Cargo.lock package '{name}@{version}' uses unsupported source '{source}'")
}
