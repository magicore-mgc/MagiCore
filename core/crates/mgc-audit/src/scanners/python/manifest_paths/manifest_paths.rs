use std::path::Path;

/// Find the canonical requirements file first, then sorted variants.
/// Tìm requirements chuẩn trước, sau đó đến các biến thể theo thứ tự ổn định.
pub fn find_requirements_file(project_root: &Path) -> Option<String> {
    let canonical = project_root.join("requirements.txt");
    if canonical.is_file() {
        return Some("requirements.txt".to_string());
    }
    let mut variants: Vec<String> = std::fs::read_dir(project_root)
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter_map(|e| e.file_name().to_str().map(str::to_string))
        .filter(|n| n.starts_with("requirements") && n.ends_with(".txt"))
        .collect();
    variants.sort();
    variants.into_iter().next()
}

/// Find the canonical PEP 751 lockfile first, then sorted named variants.
/// Tìm lockfile PEP 751 chuẩn trước, rồi đến biến thể có tên theo thứ tự.
pub fn find_pylock_file(project_root: &Path) -> Option<String> {
    let canonical = project_root.join("pylock.toml");
    if canonical.is_file() {
        return Some("pylock.toml".to_string());
    }
    let mut variants: Vec<String> = std::fs::read_dir(project_root)
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter_map(|e| e.file_name().to_str().map(str::to_string))
        .filter(|n| n == "pylock.toml" || (n.starts_with("pylock.") && n.ends_with(".toml")))
        .collect();
    variants.sort();
    variants.into_iter().next()
}
