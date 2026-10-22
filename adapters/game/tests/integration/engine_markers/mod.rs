//! Engine Markers owner wiring — wiring cho owner engine markers.

#[path = "engine_markers.rs"]
mod engine_markers;
pub use engine_markers::{
    detect_bevy_via_cargo_toml, detect_godot_via_project_godot, detect_unity_via_packages_manifest,
    detect_unreal_via_uproject,
};
