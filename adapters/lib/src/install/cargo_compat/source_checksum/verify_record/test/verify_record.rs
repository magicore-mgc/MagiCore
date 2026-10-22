//! Verify checksum creation, archive binding, tamper detection and case aliases.
//! Kiểm tra tạo checksum, binding archive, phát hiện sửa dữ liệu và case alias.

use crate::install::cargo_compat::test::{crate_archive, crate_archive_with_readme_case_aliases};
use crate::install::cargo_compat::{verify_source_checksum, write_source_checksum};

#[test]
fn cargo_directory_source_checksum_covers_extracted_files() {
    use sha2::{Digest, Sha256};

    let archive = crate_archive("fixture", "1.0.0");
    let package_checksum = hex::encode(Sha256::digest(&archive));
    let destination = tempfile::tempdir().unwrap();
    mgc_resolver::protocols::archive::extract_crate_tar_gz(
        &archive,
        destination.path(),
        "fixture-1.0.0",
    )
    .unwrap();
    let source = destination.path().join("fixture-1.0.0");

    write_source_checksum(&source, &package_checksum, &archive).unwrap();
    let body: serde_json::Value =
        serde_json::from_slice(&std::fs::read(source.join(".cargo-checksum.json")).unwrap())
            .unwrap();
    assert_eq!(body["package"].as_str(), Some(package_checksum.as_str()));
    assert_eq!(body["files"].as_object().unwrap().len(), 2);
    verify_source_checksum(&source, &package_checksum, &archive).unwrap();

    let changed_source = "pub fn changed() {}\n";
    std::fs::write(source.join("src/lib.rs"), changed_source).unwrap();
    let error = verify_source_checksum(&source, &package_checksum, &archive).unwrap_err();
    assert!(error.to_string().contains("differ from the MGC-verified"));

    // Editing the neighboring checksum map too must not turn modified bytes into trusted source.
    // Sửa tiếp map checksum cạnh đó cũng không biến byte đã đổi thành source đáng tin.
    let mut forged = body;
    forged["files"]["src/lib.rs"] =
        serde_json::Value::String(hex::encode(Sha256::digest(changed_source.as_bytes())));
    std::fs::write(
        source.join(".cargo-checksum.json"),
        serde_json::to_vec(&forged).unwrap(),
    )
    .unwrap();
    let error = verify_source_checksum(&source, &package_checksum, &archive).unwrap_err();
    assert!(error.to_string().contains("differ from the MGC-verified"));
}

#[test]
fn cargo_directory_source_checksum_handles_identical_case_aliases() {
    use sha2::{Digest, Sha256};

    let archive = crate_archive_with_readme_case_aliases(
        b"identical readme contents\n",
        b"identical readme contents\n",
    );
    let package_checksum = hex::encode(Sha256::digest(&archive));
    let destination = tempfile::tempdir().unwrap();
    mgc_resolver::protocols::archive::extract_crate_tar_gz(
        &archive,
        destination.path(),
        "fixture-1.0.0",
    )
    .unwrap();
    let source = destination.path().join("fixture-1.0.0");

    write_source_checksum(&source, &package_checksum, &archive).unwrap();
    let body: serde_json::Value =
        serde_json::from_slice(&std::fs::read(source.join(".cargo-checksum.json")).unwrap())
            .unwrap();
    assert!(body["files"].get("README.md").is_some());
    assert!(body["files"].get("readme.md").is_some());
    verify_source_checksum(&source, &package_checksum, &archive).unwrap();
}
