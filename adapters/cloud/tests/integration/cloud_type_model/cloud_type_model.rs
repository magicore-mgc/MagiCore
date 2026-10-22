//! Cloud Type Model behavior tests.
//! Test hành vi cho nhóm cloud type model.

use mgc_cloud_adapter::CloudType;

pub fn cloud_type_as_str_values() {
    assert_eq!(CloudType::Cdk.as_str(), "cdk");
    assert_eq!(CloudType::Pulumi.as_str(), "pulumi");
    assert_eq!(CloudType::Terraform.as_str(), "terraform");
    assert_eq!(CloudType::Cloudflare.as_str(), "cloudflare");
}
