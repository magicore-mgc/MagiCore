//! Tests for typed timeout configuration failures and safe diagnostics.
//! Test lỗi typed timeout và chẩn đoán không làm lộ giá trị cấu hình.

use super::super::{
    DEFAULT_EXEC_TIMEOUT_SECS, error::TimeoutError, execution_timeout_with, parse_timeout_value,
    resolve_configured_timeout,
};
use std::time::Duration;

#[test]
fn parses_positive_timeout_seconds() {
    assert_eq!(
        parse_timeout_value("37").expect("valid test fixture"),
        Duration::from_secs(37)
    );
}

#[test]
fn absent_timeout_uses_the_named_default() {
    assert_eq!(
        resolve_configured_timeout(None).expect("valid test fixture"),
        Duration::from_secs(DEFAULT_EXEC_TIMEOUT_SECS)
    );
}

#[test]
fn zero_timeout_is_rejected() {
    assert!(matches!(
        parse_timeout_value("0"),
        Err(TimeoutError::ZeroSeconds)
    ));
}

#[test]
fn malformed_timeout_preserves_parse_source_without_echoing_value() {
    let error = parse_timeout_value("not-a-secret-timeout").expect_err("invalid test input");
    assert!(error.to_string().contains("MGC_EXEC_TIMEOUT_SECS"));
    assert!(!error.to_string().contains("not-a-secret-timeout"));
    assert!(std::error::Error::source(&error).is_some());
}

#[test]
fn explicit_timeout_takes_precedence_over_environment_configuration() {
    let timeout = Duration::from_secs(12);
    let mut environment_read = false;
    let result = execution_timeout_with(false, Some(timeout), || {
        environment_read = true;
        None
    })
    .expect("valid test fixture");
    assert_eq!(result, Some(timeout));
    assert!(
        !environment_read,
        "explicit timeout must skip environment lookup"
    );
}

#[test]
fn disabled_timeout_does_not_read_environment_configuration() {
    let mut environment_read = false;
    let result = execution_timeout_with(true, None, || {
        environment_read = true;
        None
    })
    .expect("valid test fixture");
    assert_eq!(result, None);
    assert!(
        !environment_read,
        "disabled timeout must skip environment lookup"
    );
}

#[test]
fn configured_timeout_is_used_only_when_no_explicit_value_exists() {
    let result =
        execution_timeout_with(false, None, || Some("37".into())).expect("valid test fixture");
    assert_eq!(result, Some(Duration::from_secs(37)));
}

#[cfg(unix)]
#[test]
fn non_unicode_timeout_is_rejected_without_returning_raw_value() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let error = resolve_configured_timeout(Some(OsString::from_vec(vec![0xff])))
        .expect_err("invalid test input");
    assert!(matches!(error, TimeoutError::NonUnicodeValue));
    assert!(error.to_string().contains("MGC_EXEC_TIMEOUT_SECS"));
}

#[cfg(windows)]
#[test]
fn non_unicode_timeout_is_rejected_without_returning_raw_value() {
    use std::os::windows::ffi::OsStringExt;

    let invalid_unicode = std::ffi::OsString::from_wide(&[0xd800]);
    let error = resolve_configured_timeout(Some(invalid_unicode)).expect_err("invalid test input");
    assert!(matches!(error, TimeoutError::NonUnicodeValue));
    assert!(error.to_string().contains("MGC_EXEC_TIMEOUT_SECS"));
}
