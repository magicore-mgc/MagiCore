use super::super::redact_equals_value;
use crate::sanitizer::redact_args;
use crate::sanitizer::test::support::s;

#[test]
fn redacts_equals_value() {
    let args = redact_args(&s(&["--token=abc123", "--path=./src"]));
    assert_eq!(args[0], "--token=[REDACTED]");
    assert_eq!(args[1], "--path=./src");
}

#[test]
fn redacts_key_and_auth_token() {
    let args = redact_args(&s(&["--api-key=K", "_authToken=abc", "--secret=x"]));
    assert_eq!(
        args,
        s(&[
            "--api-key=[REDACTED]",
            "_authToken=[REDACTED]",
            "--secret=[REDACTED]"
        ])
    );
}

#[test]
fn leaves_non_sensitive_equals_value_unmodified() {
    assert_eq!(
        redact_equals_value("--path=./src"),
        Some("--path=./src".to_string())
    );
}

#[test]
fn equals_values_are_not_reclassified_as_separate_sensitive_flags() {
    assert_eq!(
        redact_equals_value("--label=--token"),
        Some("--label=--token".to_string())
    );
}
