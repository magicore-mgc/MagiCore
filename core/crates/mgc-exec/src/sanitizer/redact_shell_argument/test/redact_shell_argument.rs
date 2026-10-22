use super::super::redact_shell_argument;
use crate::sanitizer::redact_args;
use crate::sanitizer::test::support::s;

#[test]
fn redacts_sensitive_flags_embedded_in_shell_command_argument() {
    let args = redact_args(&s(&[
        "-c",
        "npm install --token child-secret; echo done",
        "--plain",
    ]));
    assert_eq!(args, s(&["-c", "[REDACTED]", "--plain"]));
}

#[test]
fn redacts_shell_quoted_escaped_and_suffixed_sensitive_flags() {
    let args = redact_args(&s(&[
        "-c",
        "npm install --'token' quoted-value",
        "-c",
        "npm install --token\\=escaped-value",
        "-c",
        "npm install --tokenFile camel-value",
    ]));
    assert_eq!(
        args,
        s(&["-c", "[REDACTED]", "-c", "[REDACTED]", "-c", "[REDACTED]"])
    );
}

#[test]
fn redacts_sensitive_flags_across_shell_line_continuations() {
    let args = redact_args(&s(&[
        "-c",
        "npm install --to\\\nken line-value",
        "-c",
        "npm install --to\\\r\nken line-value",
    ]));
    assert_eq!(args, s(&["-c", "[REDACTED]", "-c", "[REDACTED]"]));
}

#[test]
fn leaves_non_shell_arguments_unmodified() {
    assert_eq!(redact_shell_argument("--plain"), None);
    assert_eq!(redact_shell_argument("npm install --path ./src"), None);
}
