// Shared command normalization and denial boundary tests.
// Test chuẩn hóa command và ranh giới từ chối chung.

use super::*;

#[test]
fn option_values_do_not_become_the_command() {
    let args = vec![
        "--manifest-path".into(),
        "Cargo.toml".into(),
        "build".into(),
    ];
    assert_eq!(
        dependency_resolution_deny_reason("cargo", &args, &[]),
        Some("Cargo compile/test commands must include --locked".into())
    );
}

#[test]
fn arguments_after_separator_do_not_satisfy_the_locked_policy() {
    let args = vec!["build".into(), "--".into(), "--locked".into()];
    assert_eq!(
        dependency_resolution_deny_reason("cargo", &args, &[]),
        Some("Cargo compile/test commands must include --locked".into())
    );
}

#[test]
fn go_environment_uses_the_last_case_insensitive_value() {
    let args = vec!["build".into(), "-mod=readonly".into()];
    let allowed_env = vec![
        ("GOPROXY".into(), "https://proxy.example".into()),
        ("goproxy".into(), "off".into()),
    ];
    let denied_env = vec![
        ("GOPROXY".into(), "off".into()),
        ("GOPROXY".into(), "https://proxy.example".into()),
    ];
    assert_eq!(
        dependency_resolution_deny_reason("go", &args, &allowed_env),
        None
    );
    assert!(dependency_resolution_deny_reason("go", &args, &denied_env).is_some());
}

#[test]
fn rustc_preserves_case_for_the_exact_read_only_host_probe() {
    let allowed = vec!["-vV".into()];
    assert_eq!(
        dependency_resolution_deny_reason("rustc", &allowed, &[]),
        None
    );

    for args in [
        vec!["-vv".into()],
        vec!["--version".into()],
        vec!["-vV".into(), "--crate-name".into(), "injected".into()],
    ] {
        assert!(
            dependency_resolution_deny_reason("rustc", &args, &[]).is_some(),
            "non-exact Rustc host probe must be denied: {args:?}"
        );
    }
}

#[test]
fn deny_error_preserves_the_public_message_prefix() {
    let error = reject_external_dependency_resolution("cargo", &["publish".into()], &[])
        .expect_err("Cargo publish must be denied");
    assert_eq!(
        error.to_string(),
        "external dependency resolution blocked: Cargo registry/package subcommands are not allowed through mgc-exec"
    );
}
