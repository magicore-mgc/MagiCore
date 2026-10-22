use super::super::super::GameOptimizerError;

#[test]
fn refusals_keep_stable_diagnostic_text() {
    let temp = tempfile::tempdir().expect("tempdir");
    assert_eq!(
        GameOptimizerError::project_not_detected(temp.path()).to_string(),
        format!(
            "game optimizer requires a detected native Bevy project; '{}' is not a detected game project",
            temp.path().display()
        )
    );
    assert!(
        GameOptimizerError::unsupported_engine("godot")
            .to_string()
            .contains("detected 'godot'")
    );
    assert_eq!(
        GameOptimizerError::native_manifest_missing().to_string(),
        "native Bevy optimizer operations require Cargo.toml at the project root"
    );
    assert_eq!(
        GameOptimizerError::dependency_section_missing().to_string(),
        "root Cargo.toml missing [dependencies]"
    );
    assert_eq!(
        GameOptimizerError::native_manifest_not_regular().to_string(),
        "native Bevy optimizer requires a regular non-symlink Cargo.toml"
    );
    assert_eq!(
        GameOptimizerError::hook_manifest_missing().to_string(),
        "game optimizer hook requires Cargo.toml"
    );
    assert_eq!(
        GameOptimizerError::hook_manifest_not_regular().to_string(),
        "game optimizer hook requires a regular non-symlink Cargo.toml"
    );
    assert!(
        GameOptimizerError::conflicting_dependency("./custom")
            .to_string()
            .contains("'./custom' != './optimizer'")
    );
    assert!(
        GameOptimizerError::manifest_round_trip_failed()
            .to_string()
            .contains("refusing to report success")
    );
}
