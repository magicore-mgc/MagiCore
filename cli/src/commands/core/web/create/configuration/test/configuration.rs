//! Root Web configuration owner tests.
//! Test owner cấu hình Web gốc.

use crate::commands::core::web::scaffold_flags::ScaffoldFlags;

#[test]
fn monorepo_alias_selects_a_frontend_and_backend() {
    let mut flags = crate::commands::core::scaffold_flags::ScaffoldFlags::default();
    super::apply_framework_alias_defaults("monorepo", &mut flags);
    let config = super::build_web_config("monorepo", "sample", &flags).unwrap();
    assert_eq!(config.sub_type, "monorepo");
    assert_eq!(config.frameworks, ["react-vite", "express"]);
}

#[test]
fn scaffold_preflight_selects_the_actual_framework_layer_for_each_core_lane() {
    let mut flags = crate::commands::core::scaffold_flags::ScaffoldFlags::default();
    super::apply_framework_alias_defaults("monorepo", &mut flags);
    let monorepo = super::build_web_config("monorepo", "sample-monorepo", &flags).unwrap();
    let monorepo_layers = super::required_web_layers_for_config(
        &monorepo,
        &super::parse_framework_request("monorepo"),
    );
    assert!(monorepo_layers.contains(&"web/monorepo/frontend/react-vite".to_string()));
    assert!(monorepo_layers.contains(&"web/monorepo/backend/node/express".to_string()));
    assert!(!monorepo_layers.contains(&"web/monorepo/backend/node/node".to_string()));

    let backend = crate::wizard::engine::ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "backend".to_string(),
        frameworks: vec!["go".to_string(), "gin".to_string()],
        project_name: "sample-gin".to_string(),
        features: vec![],
        template_dir: std::path::PathBuf::new(),
    };
    let backend_layers =
        super::required_web_layers_for_config(&backend, &super::parse_framework_request("gin"));
    assert!(backend_layers.contains(&"web/backend/go/gin".to_string()));
    assert!(!backend_layers.contains(&"web/backend/go/go".to_string()));
}

#[test]
fn shadcn_projects_tailwind_without_mutating_the_parsed_flag() {
    let flags = ScaffoldFlags {
        shadcn: true,
        ..ScaffoldFlags::default()
    };
    assert!(!flags.tailwindcss);

    let config = super::build_web_config("react", "sample-shadcn", &flags).unwrap();

    assert!(
        config
            .features
            .iter()
            .any(|feature| feature == "tailwindcss")
    );
    assert!(!flags.tailwindcss);
}
