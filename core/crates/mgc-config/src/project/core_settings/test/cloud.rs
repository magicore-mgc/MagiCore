//! Cloud settings serde contract tests.
//! Test contract serde cho cấu hình Cloud.

use super::super::CloudConfig;
use crate::project::ProjectConfig;

#[test]
fn missing_type_defaults_and_explicit_type_roundtrips() {
    let empty: CloudConfig =
        serde_json::from_str("{}").expect("empty Cloud config should deserialize");
    assert_eq!(empty, CloudConfig::default());

    let config = CloudConfig {
        r#type: "terraform".to_string(),
    };
    let value = serde_json::to_value(&config).expect("Cloud config should serialize");
    assert_eq!(value, serde_json::json!({"type": "terraform"}));
    assert_eq!(
        serde_json::from_value::<CloudConfig>(value).expect("Cloud config should deserialize"),
        config
    );
}

#[test]
fn cloud_scaffold_sets_type() {
    let cdk = ProjectConfig::from_scaffold("m", "clo", "", vec!["cdk".to_string()], "", vec![]);
    assert_eq!(cdk.cloud.unwrap().r#type, "cdk");
    let none = ProjectConfig::from_scaffold("n", "cloud", "", vec![], "", vec![]);
    assert_eq!(none.cloud.unwrap().r#type, "terraform");
}

#[test]
fn non_cloud_has_no_cloud_config() {
    let web = ProjectConfig::from_scaffold("o", "web", "", vec![], "", vec![]);
    assert!(web.cloud.is_none());
}
