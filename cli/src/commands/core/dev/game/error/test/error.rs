//! Tests for Game-specific error messages.

use super::super::GameDevError;

#[test]
fn unreal_error_keeps_editor_guidance() {
    let error = GameDevError::UnrealEditorGuidance {
        uproject: "Game.uproject".to_string(),
    };

    assert!(error.to_string().contains("Game.uproject"));
    assert!(error.to_string().contains("UnrealBuildTool"));
}

#[test]
fn unsupported_engine_names_the_engine() {
    let error = GameDevError::UnsupportedEngine {
        engine: "unknown".to_string(),
    };

    assert!(error.to_string().contains("unknown"));
}
