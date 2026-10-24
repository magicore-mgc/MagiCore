//! Tests the container resource name behavior.
//! Kiểm tra hành vi của owner container_resource_name.

use super::super::container_resource_name::container_resource_name;
use crate::wizard::engine::ScaffoldConfig;
use std::path::PathBuf;

fn config(core: &str, project_name: &str) -> ScaffoldConfig {
    ScaffoldConfig {
        core: core.to_owned(),
        sub_type: String::new(),
        frameworks: Vec::new(),
        project_name: project_name.to_owned(),
        features: Vec::new(),
        template_dir: PathBuf::new(),
        from_local: None,
        no_local: false,
    }
}

#[test]
fn resource_name_keeps_core_and_slugged_project_name() {
    assert_eq!(
        container_resource_name("ai", &config("ai", "Project Name")),
        "magicore-ai-project-name:local"
    );
}
