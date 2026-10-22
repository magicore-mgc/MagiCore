//! Go sumdb h1 directory-hash calculation.
//! Tính directory hash h1 theo Go sumdb.

use crate::protocols::zip_reader;
use base64::Engine as _;
use mgc_types::{MgError, MgResult};
use sha2::{Digest, Sha256};

/// Sumdb `h1:` directory hash over a module zip (dirhash.HashZip):
/// sha256 of the sorted per-file lines `"<file-sha256-hex>  <name>\n"`,
/// base64-std encoded behind the `h1:` tag.
/// Directory hash `h1:` của sumdb trên zip module (dirhash.HashZip): sha256
/// của các dòng `"<file-sha256-hex>  <name>\n"` đã sắp xếp theo tên file,
/// mã hóa base64-std sau tag `h1:`.
pub fn dirhash_zip(entries: &[zip_reader::ZipEntry]) -> MgResult<String> {
    let mut files: Vec<&zip_reader::ZipEntry> = entries.iter().collect();
    files.sort_by(|a, b| a.name.cmp(&b.name));
    let mut hasher = Sha256::new();
    for entry in files {
        // Match Go Hash1: newline names cannot be represented unambiguously in its summary.
        // Theo chuẩn Go, tên chứa newline bị từ chối thay vì tạo dòng hash nhập nhằng.
        if entry.name.contains('\n') {
            return Err(MgError::Integrity(
                "Go dirhash does not support filenames containing newlines".to_owned(),
            ));
        }
        let file_hash = hex::encode(Sha256::digest(&entry.data));
        hasher.update(format!("{file_hash}  {}\n", entry.name).as_bytes());
    }
    let digest = hasher.finalize();
    Ok(format!(
        "h1:{}",
        base64::engine::general_purpose::STANDARD.encode(digest)
    ))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
