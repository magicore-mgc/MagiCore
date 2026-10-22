#![allow(clippy::unwrap_used)]

fn write_attestation(path: &std::path::Path, key_id: &str) {
    let value = serde_json::json!({
        "core": "web",
        "config_digest": "digest",
        "project_path": "/project",
        "key_id": key_id,
        "public_key": "public-key",
        "prev_keys": [],
        "attested_at": "2026-10-17T00:00:00Z",
        "mgc_version": "1.1.0",
        "signature": "signature"
    });
    std::fs::write(path, serde_json::to_vec(&value).expect("serialize record"))
        .expect("write record");
}

#[test]
fn keeps_one_previous_key_only_when_key_id_changes() {
    let directory = tempfile::tempdir().expect("rotation directory");
    let path = directory.path().join("attestation.json");
    write_attestation(&path, "old-key");
    assert_eq!(
        super::super::previous_key_for(&path, "new-key"),
        Some(crate::attestation::PreviousKey {
            key_id: "old-key".to_owned(),
            public_key: "public-key".to_owned(),
        })
    );
    assert!(super::super::previous_key_for(&path, "old-key").is_none());
    std::fs::write(&path, b"invalid").expect("write malformed slot");
    assert!(super::super::previous_key_for(&path, "new-key").is_none());
}
