// DispatchCommand owner test — protects the route type contract.
// Test owner DispatchCommand — phân biệt tuyến lệnh dùng chung và tuyến core.
use crate::dispatch::types::{CommonCommand, CoreCommand, DispatchCommand};

#[test]
fn common_and_core_routes_remain_distinct() {
    let common = DispatchCommand::Common(CommonCommand::Verify);
    let core = DispatchCommand::Core(CoreCommand::InstallWeb {
        packages: vec!["react".into()],
        frozen: true,
        ignore_scripts: false,
        allow_scripts: false,
        prefer_dedupe: false,
        repair: false,
        offline: false,
        compat_runtime: None,
    });

    assert!(matches!(
        common,
        DispatchCommand::Common(CommonCommand::Verify)
    ));
    assert!(matches!(
        core,
        DispatchCommand::Core(CoreCommand::InstallWeb { frozen: true, .. })
    ));
}
