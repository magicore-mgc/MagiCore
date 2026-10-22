//! Ensure tree hashing rejects links and hashes only regular files.
//! Bảo đảm tree hash từ chối symlink và chỉ hash file thường.

use super::super::implementation::hash_source_files;

#[test]
fn hashes_regular_files_with_forward_slash_relative_keys() {
    use sha2::{Digest, Sha256};

    let temp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(temp.path().join("nested")).unwrap();
    let contents = b"tracked";
    std::fs::write(temp.path().join("nested/file.txt"), contents).unwrap();

    let hashes = hash_source_files(temp.path()).unwrap();
    assert_eq!(
        hashes.get("nested/file.txt"),
        Some(&hex::encode(Sha256::digest(contents)))
    );
}

#[cfg(unix)]
#[test]
fn refuses_symbolic_links_in_managed_source_tree() {
    let temp = tempfile::tempdir().unwrap();
    let outside = temp.path().join("outside.txt");
    std::fs::write(&outside, b"outside").unwrap();
    let root = temp.path().join("root");
    std::fs::create_dir(&root).unwrap();
    std::os::unix::fs::symlink(&outside, root.join("linked.txt")).unwrap();

    let error = hash_source_files(&root).unwrap_err();
    assert!(error.to_string().contains("symbolic link"));
}
