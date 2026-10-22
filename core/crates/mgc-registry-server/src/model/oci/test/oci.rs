//! Focused OCI wire-format tests.
//! Test riêng định dạng dữ liệu OCI.

use crate::model::{OciBlobUploadResponse, OciManifest};

#[test]
fn test_manifest_keeps_camel_case_and_omits_empty_optional_values() {
    let manifest: OciManifest = serde_json::from_value(serde_json::json!({
        "schemaVersion": 2,
        "mediaType": "application/vnd.oci.image.manifest.v1+json",
        "config": {
            "mediaType": "application/vnd.oci.image.config.v1+json",
            "size": 0,
            "digest": "sha256:config"
        },
        "layers": []
    }))
    .unwrap();
    let json = serde_json::to_value(manifest).unwrap();

    assert_eq!(json["schemaVersion"], 2);
    assert!(json.get("subject").is_none());
    assert!(json.get("annotations").is_none());
    assert!(json["config"].get("data").is_none());
}

#[test]
fn test_upload_response_keeps_existing_field_names() {
    let response = OciBlobUploadResponse {
        location: "/v2/pkg/blobs/uploads/id".to_string(),
        range: None,
        docker_upload_uuid: "id".to_string(),
    };
    let json = serde_json::to_value(response).unwrap();

    assert_eq!(json["location"], "/v2/pkg/blobs/uploads/id");
    assert_eq!(json["docker_upload_uuid"], "id");
    assert_eq!(json.get("range"), Some(&serde_json::Value::Null));
}
