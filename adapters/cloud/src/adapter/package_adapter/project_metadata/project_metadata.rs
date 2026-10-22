//! Resolve package-adapter capability and manifest identity — chọn capability và identity manifest.

use crate::cloud_type::CloudType;
use mgc_types::capabilities::Capability;

use super::super::super::CloudAdapter;

pub(in crate::adapter::package_adapter) fn capabilities(
    adapter: &CloudAdapter,
) -> &'static [Capability] {
    if adapter.web.is_some() {
        CloudAdapter::CAPABILITIES_WEB
    } else {
        CloudAdapter::CAPABILITIES
    }
}

pub(in crate::adapter::package_adapter) fn manifest_identity(
    adapter: &CloudAdapter,
) -> Option<mgc_types::ManifestIdentity> {
    // Source-verified manifest per cloud type; CDK delegates through package.json.
    let (language, format, relpath) = match adapter.cloud_type {
        CloudType::Cdk => ("cdk", "package.json", "package.json"),
        CloudType::Pulumi => ("pulumi", "Pulumi.yaml", "Pulumi.yaml"),
        CloudType::Terraform => ("terraform", ".terraform.lock.hcl", ".terraform.lock.hcl"),
        CloudType::Cloudflare => ("cloudflare", "wrangler.toml", "wrangler.toml"),
    };
    Some(mgc_types::ManifestIdentity {
        core: "cloud".to_string(),
        language: language.to_string(),
        format: format.to_string(),
        relpath: relpath.to_string(),
    })
}
