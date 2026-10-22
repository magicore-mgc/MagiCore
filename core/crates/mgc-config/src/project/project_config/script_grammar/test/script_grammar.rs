//! Tests for explicit and default script-grammar resolution.
//! Test resolve grammar tường minh và grammar mặc định.

use crate::project::{ProjectConfig, ScriptGrammarConfig};

#[test]
fn explicit_grammar_pin_wins() {
    let mut config = ProjectConfig::new("demo", "ai");
    config.script = Some(ScriptGrammarConfig {
        grammar: Some("ai-script-v9".to_string()),
    });
    assert_eq!(config.script_grammar(), "ai-script-v9");
}

#[test]
fn absent_grammar_pin_uses_ecosystem_default() {
    let config = ProjectConfig::new("demo", "ai");
    assert_eq!(config.script_grammar(), "ai-script-v1");
}
