use crate::policy::PolicyContext;

#[test]
fn policy_context_retains_lane_identity() {
    let value = PolicyContext {
        framework: "next".into(),
        version: "16.3.8".into(),
        os: "darwin".into(),
        operation: "dev".into(),
    };
    assert_eq!(
        (
            value.framework.as_str(),
            value.version.as_str(),
            value.os.as_str(),
            value.operation.as_str()
        ),
        ("next", "16.3.8", "darwin", "dev")
    );
}
