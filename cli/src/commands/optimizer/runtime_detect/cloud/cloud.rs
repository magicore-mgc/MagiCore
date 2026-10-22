//! Cloud IaC runtime recognition.
use super::super::project_text;
use std::path::Path;

pub(in crate::commands::optimizer) fn detect_cloud_node_runtime(project_root: &Path) -> bool {
    let Some(content) = project_text(project_root, "package.json") else {
        return false;
    };
    let Ok(package) = serde_json::from_str::<serde_json::Value>(&content) else {
        return false;
    };
    let dependency_names = ["dependencies", "devDependencies", "optionalDependencies"]
        .into_iter()
        .filter_map(|section| package.get(section).and_then(serde_json::Value::as_object));
    let dependency_names: Vec<_> = dependency_names
        .flat_map(|dependencies| dependencies.keys().map(String::as_str))
        .collect();

    let has_pulumi_sdk = project_text(project_root, "Pulumi.yaml").is_some()
        && dependency_names.contains(&"@pulumi/pulumi");
    let has_cdk_node_sdk = dependency_names
        .iter()
        .any(|name| *name == "aws-cdk-lib" || name.starts_with("@aws-cdk/"));

    has_pulumi_sdk || has_cdk_node_sdk
}
