use crate::allowlist::parse_simple_script;

#[test]
fn parses_one_command_without_shell_and_preserves_quoted_argument() {
    let (program, args) =
        parse_simple_script(r#"node "./scripts/build app.js" --flag value"#).unwrap();
    assert_eq!(program, "node");
    assert_eq!(args, vec!["./scripts/build app.js", "--flag", "value"]);
}
