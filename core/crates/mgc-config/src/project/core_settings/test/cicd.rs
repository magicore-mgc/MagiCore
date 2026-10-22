//! CICD settings serde contract tests.
//! Test contract serde cho cấu hình CICD.

use super::super::CicdConfig;
use crate::project::ProjectConfig;

#[test]
fn missing_provider_defaults_and_explicit_provider_roundtrips() {
    let empty: CicdConfig =
        serde_json::from_str("{}").expect("empty CICD config should deserialize");
    assert_eq!(empty, CicdConfig::default());

    let config = CicdConfig {
        provider: "github-actions".to_string(),
    };
    let value = serde_json::to_value(&config).expect("CICD config should serialize");
    assert_eq!(value, serde_json::json!({"provider": "github-actions"}));
    assert_eq!(
        serde_json::from_value::<CicdConfig>(value).expect("CICD config should deserialize"),
        config
    );
}

#[test]
fn cicd_scaffold_sets_provider() {
    let cf =
        ProjectConfig::from_scaffold("p", "cicd", "", vec!["cloudflare".to_string()], "", vec![]);
    assert_eq!(cf.cicd.unwrap().provider, "cloudflare");
    let none = ProjectConfig::from_scaffold("q", "cicd", "", vec![], "", vec![]);
    assert_eq!(none.cicd.unwrap().provider, "github-actions");
}

#[test]
fn non_cicd_has_no_cicd_config() {
    let web = ProjectConfig::from_scaffold("r", "web", "", vec![], "", vec![]);
    assert!(web.cicd.is_none());
}
