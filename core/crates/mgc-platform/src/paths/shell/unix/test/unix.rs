//! Tests shell-specific startup-file selection through a temporary home.
//! Kiểm tra chọn startup file theo shell bằng home tạm.

use super::super::ensure_bin_on_path;

#[test]
fn zsh_selection_writes_only_the_zprofile_file() {
    let home = tempfile::tempdir().unwrap();
    let bin = home.path().join("managed-bin");
    std::fs::create_dir(&bin).unwrap();

    ensure_bin_on_path(home.path(), "/bin/zsh", &bin).unwrap();

    assert!(home.path().join(".zprofile").is_file());
    assert!(!home.path().join(".profile").exists());
}
