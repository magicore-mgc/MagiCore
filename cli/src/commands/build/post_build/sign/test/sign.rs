//! Signing either produces a verifiable signature or says why it did not.
//! Ký hoặc tạo chữ ký verify được, hoặc nói rõ vì sao không.

use super::super::{Signed, sign_artifact};
use crate::commands::build::post_build::sign::implementation::tests::support::support::Sandbox;

const SEED_HEX: &str = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60";

/// Write a `mgc trust` keyring and return the public key it holds.
/// Ghi keyring `mgc trust` và trả public key bên trong.
fn write_trust_keyring(sandbox: &Sandbox) -> String {
    let path = sandbox.trust_keyring();
    std::fs::create_dir_all(path.parent().expect("test")).expect("test operation must succeed");
    let key = mgc_crypto::keyring::KeyPair::generate().expect("test operation must succeed");
    let public_key = key.public_key.to_base64();
    let mut keyring = mgc_crypto::keyring::Keyring::new();
    keyring.add_key(key);
    keyring.save(&path).expect("test operation must succeed");
    public_key
}

fn write_legacy_keyring(sandbox: &Sandbox, seed_hex: &str) {
    std::fs::create_dir_all(sandbox.data_dir()).expect("test operation must succeed");
    std::fs::write(
        sandbox.legacy_keyring(),
        format!("{{\"signing_key\":\"{seed_hex}\"}}"),
    )
    .expect("test operation must succeed");
}

#[test]
fn the_trust_keyring_produces_a_detached_signature() {
    // The reported bug: the user ran `mgc trust init` and `mgc build` still did
    // not sign, because build read a different keyring.
    // (Lỗi được báo: người dùng chạy `mgc trust init` mà `mgc build` vẫn không
    // ký, vì build đọc keyring khác.)
    let sandbox = Sandbox::new();
    let expected_public = write_trust_keyring(&sandbox);
    let artifact = sandbox.root().join("sbom.output.cdx.json");
    std::fs::write(&artifact, b"{\"components\":[]}").expect("test operation must succeed");

    let Signed::Signature(path) =
        sign_artifact(&artifact, &sandbox.data_dir()).expect("a trust keyring must sign")
    else {
        panic!("a trust keyring must produce a signature");
    };
    assert!(path.is_file());
    let document: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).expect("test operation must succeed"))
            .expect("test operation must succeed");
    assert_eq!(document["algorithm"], "ed25519");
    assert_eq!(document["public_key"], expected_public);
    assert!(
        document["key_id"].as_str().is_some_and(|id| !id.is_empty()),
        "the signature must name the key that signed"
    );
}

#[test]
fn a_legacy_single_seed_keyring_still_signs() {
    let sandbox = Sandbox::new();
    write_legacy_keyring(&sandbox, SEED_HEX);
    let artifact = sandbox.root().join("a.json");
    std::fs::write(&artifact, b"{}").expect("test operation must succeed");

    let Signed::Signature(path) =
        sign_artifact(&artifact, &sandbox.data_dir()).expect("a legacy keyring must sign")
    else {
        panic!("a legacy keyring must produce a signature");
    };
    assert!(path.is_file());
}

#[test]
fn a_missing_keyring_skips_and_explains() {
    // Producing an unsigned artefact while claiming it is signed would be worse
    // than not signing at all, so the reason is carried back to the caller.
    // Tạo artefact không ký trong khi nói nó đã ký còn tệ hơn không ký, nên lý
    // do được trả về cho caller.
    let sandbox = Sandbox::new();
    let artifact = sandbox.root().join("sbom.output.cdx.json");
    std::fs::write(&artifact, b"{}").expect("test operation must succeed");

    match sign_artifact(&artifact, &sandbox.data_dir()).expect("missing key is reported") {
        Signed::Skipped(reason) => {
            // The message used to name `mgc key generate`, a command that does not
            // exist. It must now name the command that actually creates the key.
            // (Message từng nêu `mgc key generate` — lệnh không tồn tại. Nay phải
            // nêu đúng lệnh thật sự tạo khoá.)
            assert!(
                reason.contains("mgc trust init"),
                "the skip reason must say how to create the keyring: {reason}"
            );
            assert!(
                !reason.contains("mgc key generate"),
                "must not name a command that does not exist: {reason}"
            );
        }
        Signed::Signature(_) => panic!("no keyring must not produce a signature"),
    }
}

#[test]
fn a_malformed_legacy_keyring_is_reported_not_ignored() {
    let sandbox = Sandbox::new();
    write_legacy_keyring(&sandbox, "deadbeef");
    let artifact = sandbox.root().join("a.json");
    std::fs::write(&artifact, b"{}").expect("test operation must succeed");

    let error = sign_artifact(&artifact, &sandbox.data_dir())
        .expect_err("a malformed key must fail closed, not publish unsigned");

    assert!(error.to_string().contains("32 bytes"), "{error}");
    let mut signature_name = std::ffi::OsString::from("a.json");
    signature_name.push(".sig");
    assert!(!sandbox.root().join(signature_name).exists());
}
