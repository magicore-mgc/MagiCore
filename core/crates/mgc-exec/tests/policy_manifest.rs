//! Public policy manifest API smoke test — kiểm tra API manifest công khai.

use mgc_exec::policy_manifest::{
    ManifestEntry, ManifestError, POLICY_MANIFEST_SCHEMA, PolicyManifest, cache_rel_path,
    canonical_bytes, find_exception, key_id_hex, now_unix_secs, select_active_manifest,
    verify_manifest,
};

#[test]
fn public_manifest_items_keep_the_existing_module_path() {
    let entry = ManifestEntry {
        program: "npm".into(),
        args_exact: vec!["--version".into()],
        framework: "next".into(),
        framework_version: "16.3.8".into(),
        os: "darwin".into(),
        operation: "dev".into(),
        reason: "version probe".into(),
        expected: "0.0.0".into(),
    };
    let manifest = PolicyManifest {
        schema_version: POLICY_MANIFEST_SCHEMA.into(),
        id: "smoke".into(),
        issued_at: 1,
        expires_at: 2,
        key_id: "invalid".into(),
        revoked_keys: Vec::new(),
        entries: vec![entry],
        signature: String::new(),
    };
    assert!(canonical_bytes(&manifest).starts_with("MagiCore.PolicyException.v2\n"));
    assert_eq!(cache_rel_path().to_string_lossy(), "policy/exceptions.json");
    assert!(matches!(
        verify_manifest(&manifest, &[], &[], 1),
        Err(ManifestError::UntrustedKey)
    ));
    assert!(
        find_exception(
            &manifest,
            "npm",
            &["--version".into()],
            "next",
            "16.3.8",
            "darwin",
            "dev"
        )
        .is_some()
    );
    assert!(key_id_hex("invalid").is_none());
    assert!(select_active_manifest(vec![manifest], &[], &[], 1).is_none());
    let _ = now_unix_secs();
}
