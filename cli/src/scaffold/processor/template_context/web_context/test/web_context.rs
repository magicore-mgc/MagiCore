// Web context contract tests.
// Kiểm tra contract context Web.
#![allow(clippy::unwrap_used)]

use super::*;
use std::path::PathBuf;

#[test]
fn web_context_preserves_identity_typescript_and_frontend_execution_contract() {
    let config = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["react-vite".to_string()],
        project_name: "react-app".to_string(),
        features: vec![" TypeScript ".to_string()],
        template_dir: PathBuf::new(),
        from_local: None,
        no_local: false,
    };
    let context = WebTemplateContext::new(&config, &[]);
    let required = [
        "project_slug",
        "execution_compatibility_layer",
        "framework",
        "execution_architecture",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<Vec<_>>();

    let rendered = context
        .render_with_contract(
            "{{ project_slug }}|{{execution_compatibility_layer}}|{{framework}}|{{ execution_architecture }}",
            &required,
            "web-context-test",
        )
        .unwrap();

    assert_eq!(rendered, "react-app|ts|react-vite|rust-first");
}

#[test]
fn web_context_returns_typed_error_for_unsupported_required_context() {
    let config = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["react-vite".to_string()],
        project_name: "react-app".to_string(),
        features: vec![],
        template_dir: PathBuf::new(),
        from_local: None,
        no_local: false,
    };
    let context = WebTemplateContext::new(&config, &[]);
    let error = context
        .render_with_contract(
            "fixed text",
            &["unsupported_key".to_string()],
            "web-context-test",
        )
        .unwrap_err();

    assert_eq!(
        error.to_string(),
        concat!(
            "Template context 'unsupported_key' required by 'web-context-test' is not supported ",
            "by the Rust compiler context"
        )
    );
    assert!(error.downcast_ref::<TemplateContextError>().is_some());
}
