// Stable dispatch type facade test — verifies the existing public module path.
// Test facade dispatch — xác nhận đường dẫn module hiện hữu vẫn sử dụng được.
use super::{CommonCommand, CoreCommand, DispatchCommand};

#[test]
fn stable_facade_exposes_both_route_types() {
    let common = DispatchCommand::Common(CommonCommand::Verify);
    let core = DispatchCommand::Core(CoreCommand::CreateGame {
        framework: "bevy".into(),
        project_name: "fixture".into(),
        source: Default::default(),
        automation: Default::default(),
    });

    assert!(matches!(
        common,
        DispatchCommand::Common(CommonCommand::Verify)
    ));
    assert!(matches!(
        core,
        DispatchCommand::Core(CoreCommand::CreateGame { .. })
    ));
}
