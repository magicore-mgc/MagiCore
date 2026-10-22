//! Keep configured URL first and OCI second as a fallback.
//! Giữ URL cấu hình trước và OCI ở vị trí fallback thứ hai.

#![allow(clippy::unwrap_used)]

use super::{ManifestDistributionChannel, configured_channels};

#[test]
fn configured_url_precedes_oci_fallback() {
    assert_eq!(
        configured_channels(
            Some("https://policy.example".to_string()),
            Some("oci://registry/policy".to_string()),
        ),
        vec![
            ManifestDistributionChannel::Url("https://policy.example".to_string()),
            ManifestDistributionChannel::Oci("oci://registry/policy".to_string()),
        ]
    );
}

#[test]
fn configured_single_channel_and_empty_configuration_are_preserved() {
    assert_eq!(
        configured_channels(None, Some("oci://registry/policy".to_string())),
        vec![ManifestDistributionChannel::Oci(
            "oci://registry/policy".to_string()
        )]
    );
    assert!(configured_channels(None, None).is_empty());
}
