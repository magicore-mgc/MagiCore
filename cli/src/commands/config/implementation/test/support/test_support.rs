use std::fs;

/// Helper: tạo temp dir có .npmrc
pub(in crate::commands::config::implementation) fn temp_dir_with_npmrc(
    content: &str,
) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join(".npmrc"), content).unwrap();
    dir
}

/// Helper: tạo temp dir có mgc.toml
pub(in crate::commands::config::implementation) fn temp_dir_with_mgc_toml(
    content: &str,
) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("mgc.toml"), content).unwrap();
    dir
}
