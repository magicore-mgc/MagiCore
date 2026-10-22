//! Optimizer environment diagnostic safety test.
//! (Kiểm thử an toàn thông báo lỗi environment optimizer.)

use crate::commands::optimizer::env_loader::parse_env_file;
use std::fs;
use tempfile::TempDir;

#[test]
fn malformed_quote_returns_line_error_without_exposing_value() {
    let temp = TempDir::new().unwrap();
    let env_file = temp.path().join("malformed.env");
    fs::write(&env_file, "SAFE=value\nAUDIT_VALUE=\"\n").unwrap();

    let error = parse_env_file(&env_file).unwrap_err().to_string();
    assert!(error.contains("line 2"));
    assert!(!error.contains("AUDIT_VALUE"));
    assert!(!error.contains('"'));
}

#[test]
fn local_error_messages_match_legacy_optimizer_diagnostics() {
    let path = std::path::Path::new("project/.mgc-optimizer/settings.toml");
    let local = [
        super::unmatched_quote(2),
        super::directory_invalid(path),
        super::setting_invalid(path, "profile key"),
        super::config_read_failed(path),
        super::config_missing(path),
    ];
    let legacy = [
        crate::error::optimizer_env_unmatched_quote(2),
        crate::error::optimizer_config_directory_invalid(path),
        crate::error::optimizer_config_setting_invalid(path, "profile key"),
        crate::error::optimizer_config_read_failed(path),
        crate::error::optimizer_config_missing(path),
    ];

    for (local, legacy) in local.iter().zip(legacy.iter()) {
        assert_eq!(local.to_string(), legacy.to_string());
        assert!(local.downcast_ref::<super::OptimizerEnvError>().is_some());
    }
}
