//! Cloud Config behavior tests.
//! Test hành vi cho nhóm cloud config.

use crate::integration::support::tmp;
use mgc_cloud_adapter::{CloudType, detect_type};

pub fn mgc_toml_overrides_file_marker_for_cloud_type() {
    let dir = tmp("override");
    // Pulumi.yaml marks Pulumi; mgc.toml takes precedence and selects Terraform.
    // Pulumi.yaml → Pulumi, mgc.toml → Terraform (mgc.toml wins)
    std::fs::write(dir.join("Pulumi.yaml"), "name: x\nruntime: nodejs\n").unwrap();
    std::fs::write(dir.join("mgc.toml"), "[cloud]\ntype = \"terraform\"\n").unwrap();
    assert_eq!(detect_type(&dir), Some(CloudType::Terraform));
}

pub fn detect_cloudflare_via_mgc_toml() {
    let dir = tmp("mgc-cf");
    std::fs::write(dir.join("mgc.toml"), "[cloud]\ntype = \"cloudflare\"\n").unwrap();
    assert_eq!(detect_type(&dir), Some(CloudType::Cloudflare));
}

pub fn mgc_toml_recognizes_cdk_typescript_framework_alias() {
    let dir = tmp("mgc-cdk-typescript");
    std::fs::write(dir.join("mgc.toml"), "[cloud]\ntype = \"cdk-typescript\"\n").unwrap();
    assert_eq!(detect_type(&dir), Some(CloudType::Cdk));
}

pub fn mgc_toml_recognizes_terraform_gcp_framework_alias() {
    let dir = tmp("mgc-terraform-gcp");
    std::fs::write(dir.join("mgc.toml"), "[cloud]\ntype = \"terraform-gcp\"\n").unwrap();
    assert_eq!(detect_type(&dir), Some(CloudType::Terraform));
}

pub fn detect_returns_none_for_empty_dir() {
    let dir = tmp("empty");
    assert!(detect_type(&dir).is_none());
}

pub fn detect_returns_none_for_unknown_type_in_mgc_toml() {
    let dir = tmp("unknown");
    std::fs::write(dir.join("mgc.toml"), "[cloud]\ntype = \"ansible\"\n").unwrap();
    assert!(detect_type(&dir).is_none());
}
