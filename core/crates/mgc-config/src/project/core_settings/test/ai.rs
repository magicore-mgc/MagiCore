//! AI settings serde contract tests.
//! Test contract serde cho cấu hình AI.

use super::super::AiConfig;
use crate::project::ProjectConfig;

#[test]
fn missing_fields_default_and_explicit_framework_roundtrips() {
    let empty: AiConfig = serde_json::from_str("{}").expect("empty AI config should deserialize");
    assert_eq!(empty, AiConfig::default());

    let config = AiConfig {
        framework: "mcp-server".to_string(),
    };
    let value = serde_json::to_value(&config).expect("AI config should serialize");
    assert_eq!(value, serde_json::json!({"framework": "mcp-server"}));
    assert_eq!(
        serde_json::from_value::<AiConfig>(value).expect("AI config should deserialize"),
        config
    );
}

#[test]
fn ai_scaffold_sets_framework() {
    let mcp =
        ProjectConfig::from_scaffold("v", "ai", "", vec!["mcp-server".to_string()], "", vec![]);
    assert_eq!(mcp.ai.unwrap().framework, "mcp-server");
    let none = ProjectConfig::from_scaffold("w", "ai", "", vec![], "", vec![]);
    assert_eq!(none.ai.unwrap().framework, "python-agent");
}

#[test]
fn non_ai_has_no_ai_config() {
    let web = ProjectConfig::from_scaffold("x", "web", "", vec![], "", vec![]);
    assert!(web.ai.is_none());
}
