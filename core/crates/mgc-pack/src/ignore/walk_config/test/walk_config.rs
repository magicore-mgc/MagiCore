//! Public-path ignore-source precedence — precedence ignore qua API công khai.
use super::support::write;
use mgc_pack::ignore::select_files;

#[test]
fn respects_npmignore_preferring_it_over_gitignore() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(root, "package.json", "{}");
    write(root, "src/index.js", "export {}");
    write(root, "src/secret.js", "secrets");
    write(root, "src/ignored-by-git.js", "gitignored but npm-included");
    write(root, ".gitignore", "src/ignored-by-git.js\n");
    write(root, ".npmignore", "src/secret.js\n");

    let files = select_files(root).unwrap();
    let names: Vec<String> = files
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    assert!(names.contains(&"src/index.js".to_string()));
    assert!(!names.contains(&"src/secret.js".to_string()));
    assert!(names.contains(&"src/ignored-by-git.js".to_string()));
    assert!(names.contains(&"package.json".to_string()));
}

#[test]
fn falls_back_to_gitignore_without_npmignore() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(root, "package.json", "{}");
    write(root, "src/index.js", "x");
    write(root, "src/secret.js", "x");
    write(root, ".gitignore", "src/secret.js\n");

    let files = select_files(root).unwrap();
    let names: Vec<String> = files
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    assert!(names.contains(&"src/index.js".to_string()));
    assert!(!names.contains(&"src/secret.js".to_string()));
}
