//! File Markers behavior tests.
//! Test hành vi cho nhóm file markers.

use crate::integration::support::tmp;
use mgc_cloud_adapter::{CloudType, detect_type};

pub fn detect_cloudflare_via_wrangler_toml() {
    let dir = tmp("cf");
    std::fs::write(dir.join("wrangler.toml"), "name = \"worker\"\n").unwrap();
    assert_eq!(detect_type(&dir), Some(CloudType::Cloudflare));
}

pub fn detect_pulumi_via_pulumi_yaml() {
    let dir = tmp("pulumi");
    std::fs::write(dir.join("Pulumi.yaml"), "name: infra\nruntime: nodejs\n").unwrap();
    assert_eq!(detect_type(&dir), Some(CloudType::Pulumi));
}

pub fn detect_terraform_via_dot_tf_file() {
    let dir = tmp("tf");
    std::fs::write(dir.join("main.tf"), "provider \"aws\" {}\n").unwrap();
    assert_eq!(detect_type(&dir), Some(CloudType::Terraform));
}

pub fn detect_terraform_via_any_tf_extension() {
    let dir = tmp("tf2");
    std::fs::write(
        dir.join("networking.tf"),
        "resource \"aws_vpc\" \"main\" {}\n",
    )
    .unwrap();
    assert_eq!(detect_type(&dir), Some(CloudType::Terraform));
}

pub fn detect_cdk_via_package_json_aws_cdk_lib() {
    let dir = tmp("cdk");
    std::fs::write(
        dir.join("package.json"),
        r#"{"name":"infra","dependencies":{"aws-cdk-lib":"^2.0.0"}}"#,
    )
    .unwrap();
    assert_eq!(detect_type(&dir), Some(CloudType::Cdk));
}

pub fn detect_cdk_via_package_json_cdk_key() {
    let dir = tmp("cdk2");
    std::fs::write(
        dir.join("package.json"),
        r#"{"name":"infra","dependencies":{"cdk":"^2.0.0"}}"#,
    )
    .unwrap();
    assert_eq!(detect_type(&dir), Some(CloudType::Cdk));
}

pub fn package_json_without_cdk_deps_not_detected_as_cdk() {
    let dir = tmp("non-cdk");
    std::fs::write(
        dir.join("package.json"),
        r#"{"name":"web","dependencies":{"react":"^18.0.0"}}"#,
    )
    .unwrap();
    assert!(detect_type(&dir).is_none());
}
