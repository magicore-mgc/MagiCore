//! OCI Distribution media type constants tests.
//! Kiểm thử hằng media type của oci distribution

use super::*;

#[test]
fn distribution_spec_media_type_constants_are_stable() {
    assert_eq!(
        OCI_MANIFEST_V1,
        "application/vnd.oci.image.manifest.v1+json"
    );
    assert_eq!(OCI_CONFIG, "application/vnd.oci.image.config.v1+json");
    assert_eq!(
        OCI_LAYER_GZIP,
        "application/vnd.oci.image.layer.v1.tar+gzip"
    );
    assert_eq!(
        OCI_LAYER_ZSTD,
        "application/vnd.oci.image.layer.v1.tar+zstd"
    );
    assert_eq!(OCI_EMPTY_JSON, "application/vnd.oci.empty.v1+json");
}
