// UploadSession tests — serialized field contract.
// Test UploadSession — hợp đồng field được serialize.
#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::UploadSession;
use std::collections::BTreeSet;

#[test]
fn upload_session_serializes_and_deserializes_public_fields() {
    let session = UploadSession {
        upload_id: "session-1".to_string(),
        offset: 5,
        total_size: Some(12),
        digest: "sha256:full".to_string(),
        uploaded_digest: "sha256:part".to_string(),
        started_at: "2026-10-19T00:00:00Z".to_string(),
        updated_at: "2026-10-19T00:00:01Z".to_string(),
        chunk_size: 10,
    };
    let value = serde_json::to_value(&session).expect("serialize session");
    let keys = value
        .as_object()
        .expect("session object")
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();

    let expected_keys: BTreeSet<String> = [
        "chunk_size",
        "digest",
        "offset",
        "started_at",
        "total_size",
        "updated_at",
        "upload_id",
        "uploaded_digest",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    assert_eq!(keys, expected_keys);

    let decoded: UploadSession = serde_json::from_value(value).expect("deserialize session");
    assert_eq!(decoded.upload_id, "session-1");
    assert_eq!(decoded.offset, 5);
    assert_eq!(decoded.total_size, Some(12));
    assert_eq!(decoded.digest, "sha256:full");
}
