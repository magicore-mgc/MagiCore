//! Tests remote inputs rejected before network or unsafe publication.
//! Kiểm tra input mạng bị từ chối trước network hoặc publish không an toàn.

use super::support::tmp;
use crate::install::download::{ModelSource, download_model};

#[tokio::test]
async fn download_rejects_huggingface_path_traversal_before_network_or_write() {
    let tmp = tmp();
    let target = tmp.path().join("models");
    let source = ModelSource::huggingface("owner/model", "../../escaped.bin")
        .with_revision("0123456789abcdef0123456789abcdef01234567")
        .with_checksum(format!("sha256:{}", "a".repeat(64)));

    let error = download_model("model", &source, &target)
        .await
        .expect_err("remote artifact path must remain inside the model directory");
    assert!(error.to_string().contains("filename") || error.to_string().contains("path"));
    assert!(!tmp.path().join("escaped.bin").exists());
    assert!(
        !target.exists(),
        "invalid input must not create destination directories"
    );
}

#[tokio::test]
async fn remote_download_without_checksum_is_rejected_before_publish() {
    let tmp = tmp();
    let target = tmp.path().join("unverified-models");
    let source = ModelSource::Url {
        url: "https://example.invalid/model.bin".into(),
        checksum: None,
    };

    let error = download_model("m", &source, &target)
        .await
        .expect_err("remote artifacts without a trusted digest must fail closed");

    assert!(error.to_string().contains("checksum"), "{error}");
    assert!(
        !target.exists(),
        "a rejected remote artifact must not create or publish a target"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn download_rejects_symlinked_model_parent_without_touching_target() {
    use std::os::unix::fs::symlink;

    let tmp = tmp();
    let target = tmp.path().join("models");
    let outside = tmp.path().join("outside");
    std::fs::create_dir_all(&target).unwrap();
    std::fs::create_dir_all(&outside).unwrap();
    symlink(&outside, target.join("weights")).unwrap();
    let source = ModelSource::huggingface("owner/model", "weights/model.bin")
        .with_revision("0123456789abcdef0123456789abcdef01234567")
        .with_checksum(format!("sha256:{}", "a".repeat(64)));

    let error = download_model("model", &source, &target)
        .await
        .expect_err("symlinked artifact parent must be rejected");
    assert!(error.to_string().contains("real directory"));
    assert!(std::fs::read_dir(&outside).unwrap().next().is_none());
}

#[tokio::test]
async fn huggingface_defaults_are_rejected_without_network_pins() {
    let tmp = tmp();
    let source = ModelSource::huggingface("bert-base-uncased", "config.json");
    let error = download_model("bert", &source, tmp.path())
        .await
        .expect_err("unpinned Hugging Face source must fail before network access");
    assert!(error.to_string().contains("checksum"));
    assert!(
        !tmp.path().exists() || std::fs::read_dir(tmp.path()).unwrap().next().is_none(),
        "unpinned remote source must not create a published artifact"
    );
}
