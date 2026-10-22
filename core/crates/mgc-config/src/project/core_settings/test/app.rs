//! App settings serde contract tests.
//! Test contract serde cho cấu hình App.

use super::super::AppConfig;
use crate::project::ProjectConfig;

#[test]
fn defaults_and_empty_platform_omission_are_preserved() {
    let empty: AppConfig = serde_json::from_str("{}").expect("empty App config should deserialize");
    assert_eq!(empty, AppConfig::default());
    let value = serde_json::to_value(&empty).expect("default App config should serialize");
    assert_eq!(value, serde_json::json!({"language": ""}));
}

#[test]
fn explicit_language_and_platforms_roundtrip() {
    let config = AppConfig {
        language: "multi".to_string(),
        platforms: vec!["android".to_string(), "ios".to_string()],
    };
    let value = serde_json::to_value(&config).expect("App config should serialize");
    assert_eq!(
        value,
        serde_json::json!({"language": "multi", "platforms": ["android", "ios"]})
    );
    assert_eq!(
        serde_json::from_value::<AppConfig>(value).expect("App config should deserialize"),
        config
    );
}

#[test]
fn app_scaffold_sets_language() {
    let fl = ProjectConfig::from_scaffold("s", "app", "", vec!["flutter".to_string()], "", vec![]);
    assert_eq!(fl.app.unwrap().language, "flutter");
    let none = ProjectConfig::from_scaffold("t", "app", "", vec![], "", vec![]);
    assert_eq!(none.app.unwrap().language, "flutter");
}

#[test]
fn non_app_has_no_app_config() {
    let web = ProjectConfig::from_scaffold("u", "web", "", vec![], "", vec![]);
    assert!(web.app.is_none());
}
