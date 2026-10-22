//! Tests the container compose behavior.
//! Kiểm tra hành vi của owner container_compose.

use super::super::container_compose::container_compose;
#[test]
fn compose_keeps_the_mgc_image_and_shared_data_volume_contract() {
    let compose = container_compose(
        "magicore-web-demo:local",
        "ghcr.io/example/mgc:1.2.3",
        "dev",
    );

    assert!(compose.contains("MGC_IMAGE: ${MGC_IMAGE:-ghcr.io/example/mgc:1.2.3}"));
    assert!(compose.contains("image: ${MGC_IMAGE_NAME:-magicore-web-demo:local}"));
    assert!(compose.contains("command: [\"dev\"]"));
    assert!(compose.contains("mgc-data:/mgc-data"));
    assert!(compose.contains("${MGC_DATA_VOLUME:-magicore-system-data}"));
}
