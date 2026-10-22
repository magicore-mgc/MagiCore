// Core context unit tests.
// Kiểm thử unit context core.
#![allow(clippy::unwrap_used)]

use super::super::*;
use std::path::PathBuf;

#[test]
fn iot_template_context_rejects_unknown_board() {
    let config = ScaffoldConfig {
        core: "iot".to_string(),
        sub_type: String::new(),
        frameworks: vec!["esp32-rust".to_string()],
        project_name: "firmware".to_string(),
        features: vec!["unknown-board".to_string()],
        template_dir: PathBuf::new(),
    };

    let error = CoreTemplateContext::try_new(&config, "firmware", "esp32-rust")
        .expect_err("unregistered board must not receive a fallback target");
    assert!(error.to_string().contains("Unsupported board"));
}
#[test]
fn iot_template_context_rejects_board_from_another_framework() {
    let config = ScaffoldConfig {
        core: "iot".to_string(),
        sub_type: String::new(),
        frameworks: vec!["esp32-rust".to_string()],
        project_name: "firmware".to_string(),
        features: vec!["nrf52dk_nrf52832".to_string()],
        template_dir: PathBuf::new(),
    };

    let error = CoreTemplateContext::try_new(&config, "firmware", "esp32-rust")
        .expect_err("a board outside the selected framework must be rejected");
    assert!(error.to_string().contains("Unsupported board"));
}
#[test]
fn iot_template_context_uses_registered_board_target() {
    let config = ScaffoldConfig {
        core: "iot".to_string(),
        sub_type: String::new(),
        frameworks: vec!["esp32-rust".to_string()],
        project_name: "firmware".to_string(),
        features: vec!["esp32c3".to_string()],
        template_dir: PathBuf::new(),
    };

    let context = CoreTemplateContext::try_new(&config, "firmware", "esp32-rust")
        .expect("registered board should resolve");
    assert_eq!(
        context.value("target"),
        Some("riscv32imac-unknown-none-elf")
    );
}
#[test]
fn non_iot_template_context_retains_legacy_target_value() {
    let config = ScaffoldConfig {
        core: "ai".to_string(),
        sub_type: String::new(),
        frameworks: vec!["python-agent".to_string()],
        project_name: "agent".to_string(),
        features: vec!["custom-feature".to_string()],
        template_dir: PathBuf::new(),
    };

    let context = CoreTemplateContext::try_new(&config, "agent", "python-agent")
        .expect("non-IoT template context remains available");
    assert_eq!(
        context.value("target"),
        Some("riscv32imac-unknown-none-elf")
    );
}
