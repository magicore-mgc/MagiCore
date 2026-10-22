//! File integrity tests.
//! Test kiểm tra toàn vẹn file.

use super::super::super::file_integrity::git_blob_sha1;
use super::super::super::hfhub::HfHubProtocol;
use super::super::super::model_file::ModelFile;
use mgc_types::MgError;

fn file(sha256: Option<String>, git_blob_sha1: String) -> ModelFile {
    ModelFile {
        path: "weights.bin".to_string(),
        size: None,
        sha256,
        git_blob_sha1,
        url: String::new(),
    }
}

#[test]
fn git_blob_id_matches_known_object_and_sha256_is_preferred() {
    assert_eq!(
        git_blob_sha1(b"hello"),
        "b6fc4c620b67d95f953a5c1c1230aaab5db5a1b0"
    );
    let bytes = b"model bytes";
    let protocol = HfHubProtocol::new("https://huggingface.co");
    let sha256 = crate::protocols::sha256_hex(bytes);
    protocol
        .verify_file(&file(Some(sha256), "wrong-git-id".to_string()), bytes)
        .expect("test operation must succeed");
}

#[test]
fn integrity_mismatch_keeps_integrity_error_category() {
    let protocol = HfHubProtocol::new("https://huggingface.co");
    let error = protocol
        .verify_file(
            &file(Some("bad".to_string()), "wrong".to_string()),
            b"model bytes",
        )
        .expect_err("test operation must fail");
    assert!(matches!(error, MgError::Integrity(_)));
}
