//! The output SBOM describes exactly what the directory ships.
//! SBOM output mô tả đúng những gì thư mục phát hành.

use super::super::{hash_output, to_cyclonedx};
use sha2::{Digest, Sha256};

#[test]
fn every_file_is_hashed_with_its_relative_path() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("assets")).unwrap();
    std::fs::write(dir.path().join("index.html"), b"<html></html>").unwrap();
    std::fs::write(dir.path().join("assets/app.js"), b"console.log(1)").unwrap();

    let files = hash_output(dir.path()).unwrap();
    let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
    assert_eq!(paths, vec!["assets/app.js", "index.html"]);

    let js = files.iter().find(|f| f.path.ends_with("app.js")).unwrap();
    assert_eq!(js.sha256, hex(&Sha256::digest(b"console.log(1)")));
    assert_eq!(js.bytes, 14);
}

#[test]
fn the_document_names_every_component() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.js"), b"a").unwrap();
    let files = hash_output(dir.path()).unwrap();
    let document = to_cyclonedx("app", "1.0.0", &files);
    assert_eq!(document["specVersion"], "1.5");
    assert_eq!(document["components"].as_array().unwrap().len(), 1);
    assert_eq!(document["components"][0]["hashes"][0]["alg"], "SHA-256");
}

#[test]
fn a_symlink_outside_the_output_directory_is_not_described() {
    // An artefact must not claim a file it does not ship.
    // Artefact không được nhận một file nó không phát hành.
    let outside = tempfile::tempdir().unwrap();
    std::fs::write(outside.path().join("secret.txt"), b"not shipped").unwrap();
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("real.js"), b"r").unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(
        outside.path().join("secret.txt"),
        dir.path().join("link.js"),
    )
    .unwrap();

    let files = hash_output(dir.path()).unwrap();
    assert!(
        files.iter().all(|f| f.path != "link.js"),
        "a symlink out of the output must be skipped: {files:?}"
    );
}

/// Lowercase hex encoding of a digest.
/// Mã hoá hex chữ thường của một digest.
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
