//! Shared canonical temporary-directory and metadata fixtures for cache tests.
//! Fixture metadata và thư mục tạm canonical dùng chung cho cache test.

pub(super) fn tempdir_real() -> std::io::Result<tempfile::TempDir> {
    let canonical = std::env::temp_dir().canonicalize()?;
    tempfile::tempdir_in(canonical)
}

#[allow(dead_code)] // Shared fixture is intentionally unused by some owner-specific test modules.
pub(super) fn seed_shared_metadata(
    root: &std::path::Path,
    package: &str,
    payload: serde_json::Value,
) {
    let path = root
        .join("metadata")
        .join("v2")
        .join("http___127_0_0_1_9")
        .join(package)
        .join("metadata.json");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, serde_json::to_vec(&payload).unwrap()).unwrap();
}
