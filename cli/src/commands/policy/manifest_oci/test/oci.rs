//! Verify policy OCI artifact type and layer restrictions.
//! Xác minh media type và quy tắc layer của artifact OCI policy.

#![allow(clippy::unwrap_used)]

use super::validate_policy_artifact;
use crate::commands::policy::manifest_distribution::error::PolicyManifestDistributionError;

#[test]
fn rejects_unexpected_media_type_and_nonempty_layers() {
    assert!(matches!(
        validate_policy_artifact("application/json", true),
        Err(PolicyManifestDistributionError::OciMediaType(_))
    ));
    assert!(matches!(
        validate_policy_artifact(crate::commands::policy::POLICY_OCI_CONFIG_MEDIA_TYPE, false),
        Err(PolicyManifestDistributionError::OciLayers)
    ));
}
