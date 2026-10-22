use crate::commands::launcher_policy::{LauncherPolicy, Runtime};

#[test]
fn test_node_rejects_eval() {
    let policy = LauncherPolicy::dev_server(Runtime::Node);
    assert!(policy.validate_args(&["--eval", "console.log(1)"]).is_err());
    assert!(policy.validate_args(&["-e", "console.log(1)"]).is_err());
    assert!(policy.validate_args(&["--print", "1"]).is_err());
    assert!(policy.validate_args(&["-p", "1"]).is_err());
}
