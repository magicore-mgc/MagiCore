//! Contract tests for script grammar defaults and serialization.
//! Test contract cho default và serialization grammar script.

use super::super::script_grammar::{ScriptGrammarConfig, default_script_grammar};

#[test]
fn grammar_defaults_cover_every_named_ecosystem_and_alias() {
    for (ecosystem, expected) in [
        ("ai", "ai-script-v1"),
        ("app", "app-script-v1"),
        ("lib", "lib-script-v1"),
        ("game", "game-script-v1"),
        ("iot", "iot-script-v1"),
        ("cloud", "cloud-script-v1"),
        ("clo", "cloud-script-v1"),
        ("cicd", "cicd-script-v1"),
        ("hardware", "hardware-script-v1"),
        ("web", "web-script-v1"),
        ("unknown-eco", "web-script-v1"),
    ] {
        assert_eq!(default_script_grammar(ecosystem), expected);
    }
}

#[test]
fn grammar_pin_defaults_to_none_and_serializes_only_when_present() {
    let absent: ScriptGrammarConfig = serde_json::from_str("{}").expect("missing pin defaults");
    assert_eq!(absent.grammar, None);
    assert_eq!(
        serde_json::to_string(&absent).expect("serialize absent pin"),
        "{}"
    );

    let pinned = ScriptGrammarConfig {
        grammar: Some("ai-script-v1".to_string()),
    };
    assert_eq!(
        serde_json::to_string(&pinned).expect("serialize pin"),
        "{\"grammar\":\"ai-script-v1\"}"
    );
}
