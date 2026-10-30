//! Extracts a verified crate into Cargo's directory-source layout.
//! Giải nén crate đã xác minh vào layout directory-source của Cargo.
//!
//! Extraction replaces any stale source tree first: a partially materialized
//! directory (for example a crashed install) must never be reused, because the
//! extractor refuses to overwrite colliding paths and would leave a mixed tree
//! behind.
//! Giải nén thay thế source cũ trước tiên: thư mục materialize dở dang (ví dụ do
//! install bị ngắt) không bao giờ được tái sử dụng, vì extractor từ chối ghi đè
//! đường dẫn trùng và sẽ để lại cây lai.

use super::MaterializeResult;
use super::source_readiness::{
    has_cargo_source_marker, registry_package_dir, registry_source_root, write_cargo_source_marker,
};
use crate::protocols::ResolvedEntry;
use std::path::{Path, PathBuf};

/// Materialize one verified crate and leave Cargo's readiness marker behind.
///
/// A caller that sees the marker on return may treat the source directory as
/// complete. Extraction errors keep the incomplete directory (with no marker),
/// so the next install must rebuild it instead of trusting it.
/// Materialize một crate đã xác minh và để lại marker sẵn sàng của Cargo. Caller
/// thấy marker khi hàm trả về có thể coi thư mục source là hoàn tất. Lỗi giải
/// nén giữ lại thư mục dở dang (không marker), nên lần install sau phải dựng lại
/// thay vì tin nó.
pub(in crate::protocols::crates) fn materialize(
    entry: &ResolvedEntry,
    bytes: &[u8],
    cargo_home: &std::path::Path,
) -> MaterializeResult<PathBuf> {
    let cache_dir = cargo_home.join("registry").join("cache");
    std::fs::create_dir_all(&cache_dir)?;
    let crate_path = cache_dir.join(format!("{}-{}.crate", entry.name, entry.version));
    std::fs::write(&crate_path, bytes)?;

    let registry_dir = registry_source_root(cargo_home);
    let package_dir = registry_package_dir(&entry.name, &entry.version);
    let source_root = registry_dir.join(&package_dir);
    if source_root.try_exists()? {
        remove_stale_source(&source_root)?;
    }

    crate::protocols::archive::extract_crate_tar_gz(bytes, &registry_dir, &package_dir)?;
    write_cargo_source_marker(&source_root)?;
    debug_assert!(has_cargo_source_marker(&source_root));
    Ok(crate_path)
}

/// Remove a stale source tree, refusing anything that is not a real directory.
/// Xoá source cũ, từ chối mọi thứ không phải thư mục thường.
fn remove_stale_source(source_root: &Path) -> MaterializeResult<()> {
    let metadata = std::fs::symlink_metadata(source_root)?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "refusing to replace non-directory Cargo registry source '{}'",
                source_root.display()
            ),
        )
        .into());
    }
    std::fs::remove_dir_all(source_root)?;
    Ok(())
}
