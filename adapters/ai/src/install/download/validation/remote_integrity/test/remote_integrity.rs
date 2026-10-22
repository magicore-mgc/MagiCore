//! Tests trusted digest and immutable source-revision enforcement.
//! Kiểm tra bắt buộc digest tin cậy và revision nguồn bất biến.

use crate::install::download::{ModelSource, validate_remote_integrity};
use std::path::PathBuf;

#[test]
fn remote_integrity_requires_valid_digest_and_immutable_huggingface_revision() {
    let missing_revision = ModelSource::huggingface("owner/model", "model.safetensors")
        .with_checksum(format!("sha256:{}", "a".repeat(64)));
    assert!(
        validate_remote_integrity(&missing_revision)
            .unwrap_err()
            .to_string()
            .contains("revision")
    );

    let mutable_revision = ModelSource::HuggingFace {
        repo_id: "owner/model".into(),
        revision: Some("main".into()),
        filename: "model.safetensors".into(),
        checksum: Some(format!("sha256:{}", "a".repeat(64))),
    };
    assert!(
        validate_remote_integrity(&mutable_revision)
            .unwrap_err()
            .to_string()
            .contains("40-hex")
    );

    let invalid_checksum = ModelSource::Url {
        url: "https://example.invalid/model.bin".into(),
        checksum: Some("sha256:abc".into()),
    };
    assert!(
        validate_remote_integrity(&invalid_checksum)
            .unwrap_err()
            .to_string()
            .contains("64 hexadecimal")
    );

    let local = ModelSource::Local(PathBuf::from("/tmp/model.bin"));
    assert!(validate_remote_integrity(&local).is_ok());
}
