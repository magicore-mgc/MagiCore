#![allow(clippy::unwrap_used)]

use super::super::record_identity;

/// SEC-14: the install SBOM records its own identity, so the bytes on disk can
/// be checked against what the install produced.
/// (SEC-14: SBOM install ghi identity của chính nó, nên bytes trên đĩa kiểm được
/// với bản install đã sinh.)
#[test]
fn identity_sidecar_names_digest_size_and_tool_version() {
    let dir = tempfile::tempdir().unwrap();
    let sbom = dir.path().join("sbom.cyclonedx.json");
    std::fs::write(&sbom, br#"{"bomFormat":"CycloneDX"}"#).unwrap();

    record_identity(&sbom);

    let identity_path = dir.path().join("sbom.cyclonedx.json.identity.json");
    let identity: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&identity_path).unwrap()).unwrap();
    assert_eq!(identity["file"], "sbom.cyclonedx.json");
    assert_eq!(identity["bytes"], 25);
    assert_eq!(identity["tool_version"], env!("CARGO_PKG_VERSION"));
    let digest = identity["blake3"].as_str().unwrap();
    assert_eq!(digest.len(), 64, "BLAKE3 hex digest is 64 characters");
    assert_eq!(
        digest,
        mgc_crypto::blake3_signer::Blake3Hasher::hash_file(&sbom)
            .unwrap()
            .to_hex(),
        "the recorded digest must be the file's digest"
    );
}

/// A missing file never leaves a sidecar claiming a digest.
/// (File thiếu không bao giờ để lại sidecar khẳng định một digest.)
#[test]
fn missing_sbom_writes_no_identity() {
    let dir = tempfile::tempdir().unwrap();
    let sbom = dir.path().join("absent.json");
    record_identity(&sbom);
    assert!(!dir.path().join("absent.json.identity.json").exists());
}
