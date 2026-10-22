use super::super::is_valid_env_key;

#[test]
fn validates_ascii_identifier_start_and_continuation_rules() {
    for valid in ["A", "_", "MGC_PORT2", "PATH"] {
        assert!(is_valid_env_key(valid));
    }
    for invalid in ["", "2PORT", "HAS-DASH", "HAS SPACE", "NON_ASCII_é"] {
        assert!(!is_valid_env_key(invalid));
    }
}
