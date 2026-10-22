#![allow(clippy::unwrap_used)]

#[test]
fn stores_the_signed_attestation_record() {
    let directory = tempfile::tempdir().expect("attestation directory");
    let key_pair = mgc_crypto::keyring::KeyPair::generate().expect("generate signing key");
    let slot = directory.path().join("identity");
    let path = slot.join(crate::attestation::ATTESTATION_FILE);
    let attestation = super::super::sign_and_store(
        directory.path(),
        "web",
        "digest",
        &key_pair,
        Vec::new(),
        &slot,
        &path,
    )
    .expect("sign and store");
    let stored: crate::attestation::Attestation =
        serde_json::from_str(&std::fs::read_to_string(&path).expect("read stored attestation"))
            .expect("parse stored attestation");
    assert_eq!(stored, attestation);
    assert_eq!(attestation.core, "web");
}
