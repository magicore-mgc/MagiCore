use crate::policy_manifest::{ManifestEntry, POLICY_MANIFEST_SCHEMA, PolicyManifest};

#[test]
fn envelope_keeps_schema_identity_times_key_revocations_entries_and_signature() {
    let manifest = PolicyManifest {
        schema_version: POLICY_MANIFEST_SCHEMA.into(),
        id: "id".into(),
        issued_at: 1,
        expires_at: 2,
        key_id: "key".into(),
        revoked_keys: vec!["0123456789abcdef".into()],
        entries: vec![ManifestEntry {
            program: "npm".into(),
            args_exact: vec![],
            framework: "next".into(),
            framework_version: "1".into(),
            os: "linux".into(),
            operation: "dev".into(),
            reason: "why".into(),
            expected: "out".into(),
        }],
        signature: "sig".into(),
    };
    assert_eq!(manifest.schema_version, POLICY_MANIFEST_SCHEMA);
    assert_eq!(manifest.id, "id");
    assert_eq!((manifest.issued_at, manifest.expires_at), (1, 2));
    assert_eq!(manifest.key_id, "key");
    assert_eq!(manifest.revoked_keys[0], "0123456789abcdef");
    assert_eq!(manifest.entries[0].program, "npm");
    assert_eq!(manifest.signature, "sig");
}
