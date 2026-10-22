//! Tests for this runtime-detection owner.

use crate::commands::optimizer::runtime_detect::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_detect_cloud_nodejs_projects_for_cloud_runtime() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("Pulumi.yaml"),
        "name: demo\nruntime: nodejs\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"dependencies":{"@pulumi/pulumi":"^3.0.0"}}"#,
    )
    .unwrap();

    for core in ["cloud", "clo"] {
        assert_eq!(
            detect_runtimes(dir.path(), core),
            vec![DetectedRuntime::NodeJs {
                package_manager: PackageManager::Npm
            }]
        );
    }
}

#[test]
fn test_detect_cloud_cdk_nodejs_runtime() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("cdk.json"), r#"{"app":"node app.js"}"#).unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"dependencies":{"aws-cdk-lib":"^2.0.0"}}"#,
    )
    .unwrap();

    assert_eq!(
        detect_runtimes(dir.path(), "cloud"),
        vec![DetectedRuntime::NodeJs {
            package_manager: PackageManager::Npm
        }]
    );
}

#[test]
fn test_detect_cloud_nodejs_sdk_from_dev_dependencies() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("Pulumi.yaml"),
        "name: demo\nruntime: nodejs\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"devDependencies":{"@pulumi/pulumi":"^3.0.0"}}"#,
    )
    .unwrap();

    assert_eq!(
        detect_runtimes(dir.path(), "cloud"),
        vec![DetectedRuntime::NodeJs {
            package_manager: PackageManager::Npm
        }]
    );
}

#[test]
fn test_detect_cloud_does_not_infer_node_from_cdk_manifest_alone() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("cdk.json"), r#"{"app":"python app.py"}"#).unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"dependencies":{"react":"^19"}}"#,
    )
    .unwrap();

    assert_eq!(
        detect_runtimes(dir.path(), "cloud"),
        vec![DetectedRuntime::Unknown]
    );
}

#[test]
fn test_detect_cloud_does_not_treat_arbitrary_node_manifest_as_iac() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"dependencies":{"react":"^19"}}"#,
    )
    .unwrap();

    assert_eq!(
        detect_runtimes(dir.path(), "cloud"),
        vec![DetectedRuntime::Unknown]
    );
}
