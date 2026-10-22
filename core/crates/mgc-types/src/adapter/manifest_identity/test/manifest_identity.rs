//! Manifest Identity owner contract test.
//! Kiểm tra hợp đồng định danh manifest.

use crate::adapter::ManifestIdentity;

#[test]
fn manifest_identity_round_trip_preserves_each_lane_fence() {
    let identity = ManifestIdentity {
        core: "app".to_string(),
        language: "flutter".to_string(),
        format: "pubspec.yaml".to_string(),
        relpath: "mobile/pubspec.yaml".to_string(),
    };
    let encoded = serde_json::to_string(&identity).unwrap();
    let decoded: ManifestIdentity = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, identity);
}
