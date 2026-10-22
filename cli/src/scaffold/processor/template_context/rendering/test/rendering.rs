// Shared context renderer tests.
// Kiểm tra renderer context dùng chung.
#![allow(clippy::unwrap_used)]

use super::*;

struct TestValues;

impl ContextValues for TestValues {
    fn value(&self, key: &str) -> Option<&str> {
        match key {
            "project_slug" => Some("demo-app"),
            "project_name" => Some("Demo App"),
            "project_package" => Some("demo_app"),
            "mode" => Some("frontend"),
            _ => None,
        }
    }
}

#[test]
fn render_contract_supports_both_token_spacing_forms() {
    let required = ["project_slug", "project_name", "project_package"]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    let rendered = render_contract(
        &TestValues,
        "{{ project_slug }}/{{project_name}}/{{ project_package }}",
        &required,
        "renderer-test",
    )
    .unwrap();

    assert_eq!(rendered, "demo-app/Demo App/demo_app");
}

#[test]
fn render_contract_rejects_undeclared_and_unsupported_tokens_with_typed_errors() {
    let undeclared = render_contract(&TestValues, "{{missing}}", &[], "renderer-test").unwrap_err();
    assert_eq!(
        undeclared.to_string(),
        "Template token 'missing' in 'renderer-test' is not declared in template.toml"
    );
    assert!(undeclared.downcast_ref::<TemplateContextError>().is_some());

    let unsupported = render_contract(
        &TestValues,
        "{{missing}}",
        &["missing".to_string()],
        "renderer-test",
    )
    .unwrap_err();
    assert_eq!(
        unsupported.to_string(),
        "Template token 'missing' in 'renderer-test' is not supported by the Rust compiler context"
    );
    assert!(unsupported.downcast_ref::<TemplateContextError>().is_some());
}

#[test]
fn render_contract_rejects_unsupported_required_context_without_a_token() {
    let error = render_contract(
        &TestValues,
        "fixed text",
        &["missing".to_string()],
        "renderer-test",
    )
    .unwrap_err();

    assert_eq!(
        error.to_string(),
        "Template context 'missing' required by 'renderer-test' is not supported by the Rust compiler context"
    );
    assert!(error.downcast_ref::<TemplateContextError>().is_some());
}

#[test]
fn target_path_replaces_project_values_in_original_order_and_keeps_unknown_tokens() {
    let rendered = render_project_path_tokens(
        "{{ project_slug }}/{{project_name}}/{{ project_package }}/{{unknown}}",
        &TestValues,
    );
    assert_eq!(rendered, "demo-app/Demo App/demo_app/{{unknown}}");
}
