//! Signing either produces a verifiable signature or says why it did not.
//! Ký hoặc tạo chữ ký verify được, hoặc nói rõ vì sao không.

use super::super::{Signed, sign_artifact};

const SEED_HEX: &str = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60";

#[test]
fn a_keyring_produces_a_detached_signature() {
    let dir = tempfile::tempdir().unwrap();
    let artifact = dir.path().join("sbom.output.cdx.json");
    std::fs::write(&artifact, b"{\"components\":[]}").unwrap();
    let keyring = dir.path().join("keyring.json");
    std::fs::write(&keyring, format!("{{\"signing_key\":\"{SEED_HEX}\"}}")).unwrap();

    let Signed::Signature(path) = sign_artifact(&artifact, &keyring).unwrap() else {
        panic!("a keyring must produce a signature");
    };
    assert!(path.is_file());
    let document: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    assert_eq!(document["algorithm"], "ed25519");
    assert!(
        document["signature"]
            .as_str()
            .is_some_and(|s| !s.is_empty())
    );
    assert!(
        document["public_key"]
            .as_str()
            .is_some_and(|s| !s.is_empty())
    );
}

#[test]
fn a_missing_keyring_skips_and_explains() {
    // Producing an unsigned artefact while claiming it is signed would be worse
    // than not signing at all, so the reason is carried back to the caller.
    // Tạo artefact không ký trong khi nói nó đã ký còn tệ hơn không ký, nên lý
    // do được trả về cho caller.
    let dir = tempfile::tempdir().unwrap();
    let artifact = dir.path().join("sbom.output.cdx.json");
    std::fs::write(&artifact, b"{}").unwrap();

    match sign_artifact(&artifact, &dir.path().join("absent.json")).unwrap() {
        Signed::Skipped(reason) => {
            assert!(reason.contains("mgc key generate"), "{reason}");
        }
        Signed::Signature(_) => panic!("no keyring must not produce a signature"),
    }
}

#[test]
fn a_malformed_keyring_is_reported_not_ignored() {
    let dir = tempfile::tempdir().unwrap();
    let artifact = dir.path().join("a.json");
    std::fs::write(&artifact, b"{}").unwrap();
    let keyring = dir.path().join("keyring.json");
    std::fs::write(&keyring, "{\"signing_key\":\"deadbeef\"}").unwrap();

    match sign_artifact(&artifact, &keyring).unwrap() {
        Signed::Skipped(reason) => assert!(reason.contains("32 bytes"), "{reason}"),
        Signed::Signature(_) => panic!("a short key must not sign"),
    }
}
