//! Updated Package owner contract test.
//! Kiểm tra hợp đồng package đã cập nhật.

use crate::adapter::UpdatedPackage;

#[test]
fn updated_package_preserves_before_and_after_versions_in_serde() {
    let package = UpdatedPackage {
        name: "sample".to_string(),
        from_version: "1.0.0".to_string(),
        to_version: "1.1.0".to_string(),
    };
    let encoded = serde_json::to_value(&package).unwrap();
    let decoded: UpdatedPackage = serde_json::from_value(encoded.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), encoded);
}
