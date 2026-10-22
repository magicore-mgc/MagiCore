//! IoT settings serde contract tests.
//! Test contract serde cho cấu hình IoT.

use super::super::IotConfig;
use crate::project::ProjectConfig;

#[test]
fn missing_fields_default_and_explicit_board_roundtrips() {
    let empty: IotConfig = serde_json::from_str("{}").expect("empty IoT config should deserialize");
    assert_eq!(empty, IotConfig::default());

    let config = IotConfig {
        framework: "esp32-rust".to_string(),
        board: "esp32c3".to_string(),
    };
    let value = serde_json::to_value(&config).expect("IoT config should serialize");
    assert_eq!(
        value,
        serde_json::json!({"framework": "esp32-rust", "board": "esp32c3"})
    );
    assert_eq!(
        serde_json::from_value::<IotConfig>(value).expect("IoT config should deserialize"),
        config
    );
}

#[test]
fn iot_scaffold_sets_framework_and_default_board() {
    let pio = ProjectConfig::from_scaffold(
        "j",
        "iot",
        "",
        vec!["platformio".to_string()],
        "",
        vec!["esp32dev".to_string()],
    );
    let cfg = pio.iot.unwrap();
    assert_eq!(cfg.framework, "platformio");
    assert_eq!(cfg.board, "esp32dev");
    let none = ProjectConfig::from_scaffold("k", "iot", "", vec![], "", vec![]);
    let none_cfg = none.iot.unwrap();
    assert_eq!(none_cfg.framework, "esp32-rust");
    assert_eq!(none_cfg.board, "esp32c3");
}

#[test]
fn non_iot_has_no_iot_config() {
    let web = ProjectConfig::from_scaffold("l", "web", "", vec![], "", vec![]);
    assert!(web.iot.is_none());
}
