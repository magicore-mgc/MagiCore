use std::path::{Path, PathBuf};

pub(in crate::install) fn unique_staging_path(dest: &Path) -> PathBuf {
    let file_name = dest
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("model.bin");
    dest.with_file_name(format!(
        ".{file_name}.mgc-download-{}",
        uuid::Uuid::new_v4()
    ))
}
/// Tách tên file từ URL (dùng cho nhánh Url).
// (Derive destination filename from the URL tail.)
pub(in crate::install) fn filename_from_url(url: &str) -> String {
    let tail = url.rsplit('/').next().unwrap_or_default();
    let clean = tail.split(['?', '#']).next().unwrap_or_default();
    if clean.is_empty() {
        "model.bin".to_string()
    } else {
        clean.to_string()
    }
}
