//! Web resource-name constraints.

use super::super::web_container_resource_name;
use crate::wizard::engine::ScaffoldConfig;
use std::path::PathBuf;

fn config(project_name: &str) -> ScaffoldConfig {
    ScaffoldConfig {
        core: "web".to_owned(),
        sub_type: String::new(),
        frameworks: Vec::new(),
        project_name: project_name.to_owned(),
        features: Vec::new(),
        template_dir: PathBuf::new(),
    }
}

#[test]
fn short_name_stays_readable_without_a_hash_suffix() {
    assert_eq!(
        web_container_resource_name(&config("Web App")),
        "magicore-web-app"
    );
}

#[test]
fn long_name_is_deterministically_truncated_to_the_dns_limit() {
    let name = "a".repeat(100);
    let resource = web_container_resource_name(&config(&name));
    let expected_hash = blake3::hash(name.as_bytes()).to_hex().to_string();

    assert_eq!(resource.len(), 63);
    assert_eq!(
        resource,
        format!("magicore-{}-{}", "a".repeat(37), &expected_hash[..16])
    );
}
