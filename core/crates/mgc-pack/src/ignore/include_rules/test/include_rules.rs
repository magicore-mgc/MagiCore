//! Public-path include/exclude behavior — hành vi include/exclude qua API công khai.
use super::support::write;
use mgc_pack::ignore::select_files;

#[test]
fn excludes_always_excluded_dirs() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(root, "package.json", "{}");
    write(root, "node_modules/lodash/index.js", "x");
    write(root, ".git/config", "x");
    write(root, ".magicore/config", "x");

    let files = select_files(root).unwrap();
    let names: Vec<String> = files
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    assert!(!names.iter().any(|n| n.starts_with("node_modules")));
    assert!(!names.iter().any(|n| n.starts_with(".git")));
    assert!(!names.iter().any(|n| n.starts_with(".magicore")));
    assert!(names.contains(&"package.json".to_string()));
}

#[test]
fn always_includes_readme_license_changelog_even_if_ignored() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(root, "package.json", "{}");
    write(root, "README.md", "readme");
    write(root, "LICENSE", "mit");
    write(root, "CHANGELOG.md", "changes");
    write(root, ".gitignore", "README.md\nLICENSE\nCHANGELOG.md\n");

    let files = select_files(root).unwrap();
    let names: Vec<String> = files
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    assert!(names.contains(&"README.md".to_string()));
    assert!(names.contains(&"LICENSE".to_string()));
    assert!(names.contains(&"CHANGELOG.md".to_string()));
}
