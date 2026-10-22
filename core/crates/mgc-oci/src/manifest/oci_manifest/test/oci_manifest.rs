//! OCI image manifest model tests.
//! Kiểm thử model oci image manifest

use super::*;
use crate::manifest::OciDescriptor;

#[test]
fn oci_wire_names_are_canonical_and_legacy_manifests_still_deserialize() {
    let manifest = OciManifest::new(
        OciDescriptor::new(
            "application/vnd.oci.image.config.v1+json".into(),
            2,
            "sha256:config".into(),
        ),
        vec![],
    );
    let canonical = serde_json::to_value(&manifest).unwrap();
    assert_eq!(canonical["schemaVersion"], 2);
    assert_eq!(
        canonical["mediaType"],
        "application/vnd.oci.image.manifest.v1+json"
    );
    assert_eq!(
        canonical["config"]["mediaType"],
        "application/vnd.oci.image.config.v1+json"
    );
    assert!(canonical.get("schema_version").is_none());
    assert!(canonical["config"].get("media_type").is_none());

    let legacy: OciManifest = serde_json::from_value(serde_json::json!({
        "schema_version": 2,
        "media_type": "application/vnd.oci.image.manifest.v1+json",
        "config": {
            "media_type": "application/vnd.oci.image.config.v1+json",
            "size": 2,
            "digest": "sha256:config"
        },
        "layers": []
    }))
    .unwrap();
    assert_eq!(legacy.schema_version, 2);
    assert_eq!(
        legacy.config.media_type,
        "application/vnd.oci.image.config.v1+json"
    );
}
