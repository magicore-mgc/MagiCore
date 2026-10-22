use crate::commands::launcher_policy::{LauncherPolicy, Runtime};

#[test]
fn test_bun_rejects_eval() {
    let policy = LauncherPolicy::dev_server(Runtime::Bun);
    assert!(policy.validate_args(&["--eval", "console.log(1)"]).is_err());
    assert!(policy.validate_args(&["-e", "console.log(1)"]).is_err());
    assert!(policy.validate_args(&["--print", "1"]).is_err());
    assert!(policy.validate_args(&["-p", "1"]).is_err());
}

#[test]
fn test_bun_allows_safe_args() {
    let policy = LauncherPolicy::dev_server(Runtime::Bun);
    assert!(policy.validate_args(&["run", "server.ts"]).is_ok());
    assert!(policy.validate_args(&["test"]).is_ok());
}
