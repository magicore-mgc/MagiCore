use crate::policy_manifest::{
    ManifestEntry, POLICY_MANIFEST_SCHEMA, PolicyManifest, canonical_bytes,
};

pub(crate) fn signed_manifest(expires_at: i64) -> (PolicyManifest, String) {
    let (seed, key_id) = test_key();
    let signer =
        mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&seed).expect("valid test fixture");
    let mut manifest = PolicyManifest {
        schema_version: POLICY_MANIFEST_SCHEMA.to_string(),
        id: "test-next-darwin".to_string(),
        issued_at: 1_000_000,
        expires_at,
        key_id: key_id.clone(),
        revoked_keys: vec![],
        entries: vec![ManifestEntry {
            program: "npm".to_string(),
            args_exact: vec!["--version".to_string()],
            framework: "next".to_string(),
            framework_version: "16.3.8".to_string(),
            os: "darwin".to_string(),
            operation: "dev".to_string(),
            reason: "framework version probe".to_string(),
            expected: "0.0.0".to_string(),
        }],
        signature: String::new(),
    };
    let signature = signer.sign_string(&canonical_bytes(&manifest));
    manifest.signature = signature.to_base64();
    (manifest, key_id)
}

pub(crate) fn test_key() -> ([u8; 32], String) {
    let seed = [7u8; 32];
    let signer =
        mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&seed).expect("valid test fixture");
    let key_id = signer.public_key().to_base64();
    (seed, key_id)
}
