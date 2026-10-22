use super::super::is_sensitive_flag;

#[test]
fn detects_sensitive_name_segments_and_prefixes() {
    for flag in [
        "--token",
        "--tokenFile",
        "--api-key",
        "_authToken",
        "--password",
    ] {
        assert!(is_sensitive_flag(flag), "expected {flag} to be sensitive");
    }
}

#[test]
fn detects_configured_short_flag() {
    assert!(is_sensitive_flag("-p"));
}

#[test]
fn ignores_non_sensitive_names_and_empty_flags() {
    for flag in ["--path", "--no-scripts", "-"] {
        assert!(!is_sensitive_flag(flag), "expected {flag} to be ordinary");
    }
}
