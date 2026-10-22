//! Covers optional artifact verification fields in serialized schema.
//! Kiểm tra field xác minh artifact tùy chọn trong schema đã serialize.

use super::super::ArtifactRef;

#[test]
fn omits_absent_optional_verification_fields() {
    let artifact = ArtifactRef {
        url: "https://example.invalid/sample.tgz".to_string(),
        size_bytes: None,
        content_hash: "blake3-test".to_string(),
        downloaded_from: "registry".to_string(),
        integrity_sri: None,
    };

    let value = serde_json::to_value(&artifact).expect("test operation must succeed");
    assert!(value.get("size_bytes").is_none());
    assert!(value.get("integrity_sri").is_none());
    assert_eq!(
        serde_json::from_value::<ArtifactRef>(value).expect("test operation must succeed"),
        artifact
    );
}
