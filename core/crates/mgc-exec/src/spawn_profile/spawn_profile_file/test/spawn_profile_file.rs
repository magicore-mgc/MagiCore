use super::super::SpawnProfileFile;

#[test]
fn optional_lock_metadata_defaults_and_is_omitted_when_absent() {
    let mut document = serde_json::json!({
        "schema_version": "spawn-profile-v1",
        "mgc_version": "1.1.0",
        "captured_at": 12,
        "profile": {
            "framework": "next",
            "version": "16.3.8",
            "os": "darwin",
            "operation": "dev",
            "allowed_pm_invocations": [],
            "requires_pm": []
        }
    });
    let profile: SpawnProfileFile = serde_json::from_value(document.clone()).unwrap();
    assert!(profile.corpus_id.is_empty());
    assert_eq!(profile.lock_digest, None);
    document["corpus_id"] = serde_json::json!("capture-7");
    let profile: SpawnProfileFile = serde_json::from_value(document).unwrap();
    assert_eq!(profile.corpus_id, "capture-7");
    assert!(
        serde_json::to_value(profile)
            .unwrap()
            .get("lock_digest")
            .is_none()
    );
}
