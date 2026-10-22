use crate::allowlist::parse_script_invocation;

#[test]
fn parses_leading_env_assignments_without_shell() {
    let invocation =
        parse_script_invocation(r#"NODE_ENV=production MGC_PORT=4315 vite --host localhost"#)
            .unwrap();

    assert_eq!(invocation.program, "vite");
    assert_eq!(invocation.args, vec!["--host", "localhost"]);
    assert_eq!(
        invocation.env,
        vec![
            ("NODE_ENV".to_string(), "production".to_string()),
            ("MGC_PORT".to_string(), "4315".to_string())
        ]
    );
}

#[test]
fn rejects_env_only_scripts() {
    let err = parse_script_invocation("NODE_ENV=production").unwrap_err();
    assert!(err.to_string().contains("missing a program"));
}
