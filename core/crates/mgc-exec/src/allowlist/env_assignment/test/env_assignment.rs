use super::super::{is_env_assignment, parse_env_assignment};

#[test]
fn environment_assignment_parser_validates_key_and_preserves_value() {
    assert!(is_env_assignment("MODE=prod"));
    assert!(!is_env_assignment("9MODE=prod"));
    assert_eq!(parse_env_assignment("MODE=a=b").unwrap(), ("MODE", "a=b"));
    assert!(parse_env_assignment("9MODE=prod").is_err());
}
