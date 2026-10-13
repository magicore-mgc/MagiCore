#![allow(clippy::unwrap_used)]

use std::process::Command;

#[test]
fn login_rejects_password_command_argument_without_echoing_it() {
    let secret = "never-print-this-password";
    let output = Command::new(env!("CARGO_BIN_EXE_mgc"))
        .args([
            "login",
            "--registry",
            "https://registry.example/",
            "--username",
            "audit-user",
            "--password",
            secret,
        ])
        .output()
        .expect("spawn mgc");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(!output.status.success(), "password args must be rejected");
    assert!(
        text.contains("unexpected argument") || text.contains("unexpected argument '--password'"),
        "{text}"
    );
    assert!(
        !text.contains(secret),
        "password leaked in CLI output: {text}"
    );
}
