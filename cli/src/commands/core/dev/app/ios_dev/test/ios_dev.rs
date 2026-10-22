use super::*;

#[test]
fn xcodebuild_arguments_keep_scheme_destination_and_lane_action_order() {
    assert_eq!(
        xcodebuild_args("Sample".to_string(), "SIM-123".to_string(), "build"),
        ["-scheme", "Sample", "-destination", "id=SIM-123", "build"]
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        xcodebuild_args("Sample".to_string(), "SIM-123".to_string(), "run").last(),
        Some(&"run".to_string())
    );
}
