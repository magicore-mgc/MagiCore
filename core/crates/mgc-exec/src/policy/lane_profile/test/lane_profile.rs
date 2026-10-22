use crate::policy::LaneProfile;

#[test]
fn lane_profile_keeps_exact_lane_and_invocation_fields() {
    let value = LaneProfile {
        framework: "next".into(),
        version: "16.3.8".into(),
        os: "darwin".into(),
        operation: "dev".into(),
        allowed_pm_invocations: vec!["npm --version".into()],
        requires_pm: vec!["npm".into()],
    };
    assert_eq!(value.framework, "next");
    assert_eq!(value.version, "16.3.8");
    assert_eq!(value.os, "darwin");
    assert_eq!(value.operation, "dev");
    assert_eq!(value.allowed_pm_invocations[0], "npm --version");
    assert_eq!(value.requires_pm[0], "npm");
}
