//! Game settings serde contract tests.
//! Test contract serde cho cấu hình Game.

use super::super::GameConfig;
use crate::project::ProjectConfig;

#[test]
fn missing_engine_defaults_and_explicit_engine_roundtrips() {
    let empty: GameConfig =
        serde_json::from_str("{}").expect("empty Game config should deserialize");
    assert_eq!(empty, GameConfig::default());

    let config = GameConfig {
        engine: "bevy".to_string(),
    };
    let value = serde_json::to_value(&config).expect("Game config should serialize");
    assert_eq!(value, serde_json::json!({"engine": "bevy"}));
    assert_eq!(
        serde_json::from_value::<GameConfig>(value).expect("Game config should deserialize"),
        config
    );
}

#[test]
fn game_scaffold_sets_engine_default_bevy() {
    let godot =
        ProjectConfig::from_scaffold("g", "game", "", vec!["godot".to_string()], "", vec![]);
    assert_eq!(godot.game.unwrap().engine, "godot");
    let none = ProjectConfig::from_scaffold("h", "game", "", vec![], "", vec![]);
    assert_eq!(none.game.unwrap().engine, "bevy");
}

#[test]
fn non_game_has_no_game_config() {
    let web = ProjectConfig::from_scaffold("i", "web", "", vec![], "", vec![]);
    assert!(web.game.is_none());
}
