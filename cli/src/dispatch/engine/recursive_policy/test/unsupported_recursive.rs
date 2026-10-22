// Unsupported recursive cases — preserve refusal for both absent and unsupported commands.
// Ca recursive không hỗ trợ — giữ từ chối khi thiếu command hoặc command không hỗ trợ.
use super::super::{recursive_supported, reject_unsupported_recursive};
use super::parse_command;

#[test]
fn recursive_rejects_an_unsupported_command_with_its_stable_label() {
    let command = parse_command("init", false, false);
    assert!(!recursive_supported(&command));
    assert_eq!(
        reject_unsupported_recursive(Some(&command))
            .unwrap_err()
            .to_string(),
        "--recursive is not implemented for 'init' yet. Beta safety rule: refusing a silent no-op on workspace commands."
    );
}

#[test]
fn recursive_without_a_command_keeps_its_stable_diagnostic() {
    assert_eq!(
        reject_unsupported_recursive(None).unwrap_err().to_string(),
        "--recursive is declared on the CLI surface but workspace recursion is not implemented yet. Refusing to pretend it ran."
    );
}
