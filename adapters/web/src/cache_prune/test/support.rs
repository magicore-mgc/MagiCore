//! Canonical temporary paths for Web cache pruning tests.
//! Đường dẫn tạm canonical cho test prune cache Web.

pub(super) fn tempdir_real() -> std::io::Result<tempfile::TempDir> {
    let canonical = std::env::temp_dir().canonicalize()?;
    tempfile::tempdir_in(canonical)
}
