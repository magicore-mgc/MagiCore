use crate::sanitizer::redact_args;
use crate::sanitizer::test::support::s;

#[test]
fn redacts_separate_value() {
    let args = redact_args(&s(&["--token", "supersecret", "-v"]));
    assert_eq!(args, s(&["--token", "[REDACTED]", "-v"]));
}

#[test]
fn redacts_short_p_password() {
    let args = redact_args(&s(&["-p", "hunter2"]));
    assert_eq!(args, s(&["-p", "[REDACTED]"]));
}

#[test]
fn keeps_plain_args() {
    let args = redact_args(&s(&["--no-scripts", "build", "src/main.rs"]));
    assert_eq!(args, s(&["--no-scripts", "build", "src/main.rs"]));
}

#[test]
fn dangling_sensitive_flag_gets_redacted() {
    let args = redact_args(&s(&["--token"]));
    assert_eq!(args, s(&["--token", "[REDACTED]"]));
}

#[test]
fn empty_args_pass_through() {
    assert_eq!(redact_args(&[]), Vec::<String>::new());
}
