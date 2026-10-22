//! Tests for hook argv tokenization and shell syntax rejection.
//! Test tách argv hook và từ chối cú pháp shell.

use super::super::parse_hook_command;

#[test]
fn quoted_arguments_are_split_without_shell_execution() {
    assert_eq!(
        parse_hook_command("tool --label 'two words' \"three words\"").unwrap(),
        vec![
            "tool".to_string(),
            "--label".to_string(),
            "two words".to_string(),
            "three words".to_string()
        ]
    );
}

#[test]
fn blank_commands_have_no_argv() {
    assert!(parse_hook_command(" \t\n").unwrap().is_empty());
}

#[test]
fn shell_control_and_metacharacters_are_rejected() {
    for command in [
        "tool && next",
        "tool | next",
        "tool; next",
        "tool < input",
        "tool > output",
        "tool `id`",
        "tool $HOME",
        "tool (next)",
    ] {
        assert!(
            parse_hook_command(command).is_err(),
            "must reject {command}"
        );
    }
}

#[test]
fn unterminated_quotes_are_rejected() {
    assert!(
        parse_hook_command("tool 'unfinished")
            .unwrap_err()
            .to_string()
            .contains("unterminated quote")
    );
}
