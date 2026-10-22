//! Manifest Parsing owner wiring — wiring cho owner manifest parsing.

#[path = "manifest_parsing.rs"]
mod manifest_parsing;
pub use manifest_parsing::{
    parse_manifest_bevy_reads_cargo_toml_dependencies, parse_manifest_godot_returns_game_manifest,
};
