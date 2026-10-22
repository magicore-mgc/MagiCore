use std::path::Path;

/// Historical detector kept for migration diagnostics only; it no longer grants exec bypass.
/// Bộ nhận diện cũ chỉ còn phục vụ chẩn đoán migration, không mở khóa chạy PM.
pub(in crate::allowlist) fn is_react_native_subdir(cwd: Option<&Path>) -> bool {
    let Some(cwd) = cwd else { return false };
    let rn_ok = cwd.join("package.json").is_file()
        && std::fs::read_to_string(cwd.join("package.json"))
            .map(|s| s.contains("\"react-native\""))
            .unwrap_or(false);
    if !rn_ok {
        return false;
    }
    let mut current = cwd.parent();
    while let Some(dir) = current {
        let mgc = dir.join("mgc.toml");
        if mgc.is_file() {
            return std::fs::read_to_string(mgc)
                .map(|s| s.contains("language = \"multi\""))
                .unwrap_or(false);
        }
        current = dir.parent();
    }
    false
}
