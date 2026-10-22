//! Script policy precedence tests.
//! Test thứ tự ưu tiên của policy script.

use super::super::{ScriptVerdict, ScriptsPolicy, decide_scripts};

#[test]
fn deny_wins_across_file_and_machine_sources() {
    let file = ScriptsPolicy {
        policy: None,
        allow: vec!["package".to_string()],
        deny: vec![],
    };
    assert!(matches!(
        decide_scripts("package", "1.0.0", Some(&file), Some("denied"), false),
        ScriptVerdict::Deny("mgc trust deny")
    ));
}

#[test]
fn file_deny_precedes_database_approval_and_allow() {
    let file = ScriptsPolicy {
        policy: None,
        allow: vec![],
        deny: vec!["package@1.0.0".to_string()],
    };
    assert!(matches!(
        decide_scripts("package", "1.0.0", Some(&file), Some("approved"), false),
        ScriptVerdict::Deny("mgc.toml [scripts] deny")
    ));
}

#[test]
fn undecided_and_blanket_default_keep_existing_behavior() {
    assert!(matches!(
        decide_scripts("package", "1.0.0", None, None, false),
        ScriptVerdict::Undecided
    ));
    assert!(matches!(
        decide_scripts("package", "1.0.0", None, None, true),
        ScriptVerdict::Allow("default policy")
    ));
}
