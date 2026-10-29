#![allow(clippy::unwrap_used)]

use super::*;

fn write_signature(artifact: &Path, signer: &mgc_crypto::Ed25519Signer) {
    let bytes = std::fs::read(artifact).unwrap();
    let document = serde_json::json!({
        "file": artifact.file_name().and_then(|n| n.to_str()).unwrap_or_default(),
        "algorithm": "ed25519",
        "signature": signer.sign(&bytes).to_base64(),
        "public_key": signer.public_key().to_base64(),
    });
    std::fs::write(
        signature_path_for(artifact),
        serde_json::to_vec_pretty(&document).unwrap(),
    )
    .unwrap();
}

/// Deterministic signer so a failure names the crypto reason, not the RNG.
/// (Signer tất định để lỗi nêu lý do crypto, không phải RNG.)
fn signer(seed_byte: u8) -> mgc_crypto::Ed25519Signer {
    mgc_crypto::Ed25519Signer::from_seed(&[seed_byte; 32]).unwrap()
}

fn artifact() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("sbom.cdx.json");
    std::fs::write(&path, br#"{"bomFormat":"CycloneDX"}"#).unwrap();
    (dir, path)
}

/// SEC-15: a signature produced by the build signer verifies with the pinned
/// key, so the write-only signature now protects something.
/// (SEC-15: chữ ký do signer build tạo verify được với key đã pin, nên chữ ký
/// trước đây chỉ ghi nay bảo vệ được thật.)
#[test]
fn signature_from_the_builder_verifies() {
    let (_dir, path) = artifact();
    let signer = signer(7);
    write_signature(&path, &signer);

    let verified = verify(&path, &signer.public_key().to_base64()).expect("valid signature");
    assert_eq!(verified.artifact, std::path::absolute(&path).unwrap());
}

/// A missing `.sig` is a failure, not a silent pass.
/// (Thiếu `.sig` là lỗi, không phải cho qua im lặng.)
#[test]
fn missing_signature_is_refused() {
    let (_dir, path) = artifact();
    let signer = signer(7);
    let error = verify(&path, &signer.public_key().to_base64()).expect_err("no signature");
    assert!(error.to_string().contains("no signature"), "{error}");
}

/// Tampering the artefact after signing invalidates it.
/// (Sửa artefact sau khi ký làm mất hiệu lực.)
#[test]
fn tampered_artifact_is_refused() {
    let (_dir, path) = artifact();
    let signer = signer(7);
    write_signature(&path, &signer);
    std::fs::write(&path, br#"{"bomFormat":"CycloneDX","components":[]}"#).unwrap();

    let error = verify(&path, &signer.public_key().to_base64()).expect_err("tampered");
    assert!(error.to_string().contains("does not verify"), "{error}");
}

/// A signature from another key is refused even when it is self-consistent.
/// (Chữ ký từ key khác bị từ chối dù tự nhất quán.)
#[test]
fn unpinned_signer_is_refused() {
    let (_dir, path) = artifact();
    let stranger = signer(9);
    let trusted = signer(11);
    write_signature(&path, &stranger);

    let error = verify(&path, &trusted.public_key().to_base64()).expect_err("unpinned key");
    assert!(
        error.to_string().contains("not the pinned trust key"),
        "{error}"
    );
}

/// A signature copied from a different file name is refused.
/// (Chữ ký copy từ tên file khác bị từ chối.)
#[test]
fn signature_for_another_file_is_refused() {
    let (_dir, path) = artifact();
    let signer = signer(7);
    let bytes = std::fs::read(&path).unwrap();
    let document = serde_json::json!({
        "file": "other.cdx.json",
        "algorithm": "ed25519",
        "signature": signer.sign(&bytes).to_base64(),
        "public_key": signer.public_key().to_base64(),
    });
    std::fs::write(
        signature_path_for(&path),
        serde_json::to_vec_pretty(&document).unwrap(),
    )
    .unwrap();

    let error = verify(&path, &signer.public_key().to_base64()).expect_err("wrapped");
    assert!(error.to_string().contains("was issued for"), "{error}");
}

/// The signature path follows the signer's naming rule exactly, so verify never
/// looks at the wrong file.
/// (Đường dẫn chữ ký đúng quy tắc đặt tên của signer, nên verify không bao giờ
/// đọc nhầm file.)
#[test]
fn signature_path_matches_the_signer() {
    assert_eq!(
        signature_path_for(Path::new("dist/sbom.cdx.json")),
        PathBuf::from("dist/sbom.cdx.json.sig")
    );
    assert_eq!(
        signature_path_for(Path::new("dist/bundle")),
        PathBuf::from("dist/bundle.sig")
    );
}
