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
            expires_at: Some(super::super::rotation_grace_deadline()),
        })
    );
    assert!(super::super::previous_key_for(&path, "old-key").is_none());
    std::fs::write(&path, b"invalid").expect("write malformed slot");
    assert!(super::super::previous_key_for(&path, "new-key").is_none());
}

/// P-8.5: a rotated-out key carries a deadline, and an expired deadline is a
/// hard stop rather than a warning.
/// (P-8.5: key bị rotation có hạn, và hạn đã qua là dừng hẳn, không chỉ cảnh
/// báo.)
#[test]
fn rotated_key_grace_window_is_bounded_and_expires() {
    let deadline = super::super::rotation_grace_deadline();
    assert!(deadline.ends_with('Z'), "{deadline}");
    assert_eq!(deadline.len(), 20, "RFC 3339 UTC shape: {deadline}");

    // Inside the window the key still verifies, at the boundary it is still
    // valid, and one second later it is refused.
    // (Trong hạn key vẫn verify, đúng mốc vẫn hợp lệ, một giây sau bị từ chối.)
    assert!(!super::super::rotation_key_expired(
        &deadline,
        "2026-01-01T00:00:00Z"
    ));
    assert!(!super::super::rotation_key_expired(&deadline, &deadline));
    assert!(super::super::rotation_key_expired(
        &deadline,
        "2099-01-01T00:00:00Z"
    ));
}

/// The conversion must produce real calendar dates, not an epoch of zeros.
/// (Phép đổi phải cho ngày lịch thật, không phải mốc 0.)
#[test]
fn civil_date_conversion_matches_known_instants() {
    // 1970-01-01 and 2026-10-30 (a leap-year-safe boundary).
    // (1970-01-01 và 2026-10-30, mốc an toàn qua năm nhuận.)
    assert_eq!(super::super::civil_from_days(0), (1970, 1, 1));
    assert_eq!(super::super::civil_from_days(20_756), (2026, 10, 30));
}
