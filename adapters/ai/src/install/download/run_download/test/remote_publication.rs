//! Tests verified remote publication and preservation of prior artifacts.
//! Kiểm tra publish nguồn mạng đã xác minh và giữ artifact cũ khi lỗi.

use super::support::{serve_once, tmp};
use crate::install::download::{ModelSource, download_model};

#[tokio::test]
async fn test_url_branch_downloads_real_bytes_and_verifies_sha256() {
    let tmp = tmp();
    let target = tmp.path().join("out");
    let Some((url, server)) = serve_once(b"magic-bytes") else {
        return;
    };

    use sha2::Digest;
    let good = hex::encode(sha2::Sha256::digest(b"magic-bytes"));
    let source = ModelSource::Url {
        url,
        checksum: Some(format!("sha256:{good}")),
    };

    let (path, bytes) = download_model("m", &source, &target).await.unwrap();
    server.join().unwrap();
    assert_eq!(bytes, b"magic-bytes".len() as u64);
    assert_eq!(std::fs::read(&path).unwrap(), b"magic-bytes".to_vec());

    // Checksum đúng → file còn nguyên
    // (Correct checksum → artifact stays on disk)
    assert!(path.exists());
}

#[tokio::test]
async fn test_url_branch_wrong_checksum_preserves_existing_artifact() {
    let tmp = tmp();
    let target = tmp.path().join("out");
    std::fs::create_dir_all(&target).unwrap();
    std::fs::write(target.join("model.bin"), b"trusted-old-artifact").unwrap();
    let Some((url, server)) = serve_once(b"tampered-or-not") else {
        return;
    };

    let source = ModelSource::Url {
        url,
        checksum: Some("sha256:".to_string() + &"0".repeat(64)),
    };
    let err = download_model("m", &source, &target).await.unwrap_err();
    server.join().unwrap();

    assert!(err.to_string().contains("checksum mismatch"));
    assert_eq!(
        std::fs::read(target.join("model.bin")).unwrap(),
        b"trusted-old-artifact",
        "failed replacement must preserve the previous verified artifact"
    );
    assert!(
        std::fs::read_dir(&target).unwrap().all(|entry| !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .contains("mgc-download")),
        "failed checksum must not leak staging files"
    );
}
