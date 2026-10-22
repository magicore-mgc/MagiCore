use crate::commands::launcher_policy::{LauncherPolicy, Runtime};

#[test]
fn test_deno_rejects_allow_all() {
    let policy = LauncherPolicy::dev_server(Runtime::Deno);
    assert!(policy.validate_args(&["--eval", "console.log(1)"]).is_err());
    assert!(policy.validate_args(&["-e", "console.log(1)"]).is_err());
    assert!(policy.validate_args(&["--allow-all", "script.ts"]).is_err());
    assert!(policy.validate_args(&["-A", "script.ts"]).is_err());
}

#[test]
fn test_deno_rejects_dangerous_permissions() {
    let policy = LauncherPolicy::dev_server(Runtime::Deno);
    assert!(
        policy
            .validate_args(&["--allow-read", "script.ts"])
            .is_err()
    );
    assert!(
        policy
            .validate_args(&["--allow-write", "script.ts"])
            .is_err()
    );
    assert!(policy.validate_args(&["--allow-net", "script.ts"]).is_err());
    assert!(policy.validate_args(&["--allow-run", "script.ts"]).is_err());
    assert!(policy.validate_args(&["--allow-ffi", "script.ts"]).is_err());
    assert!(
        policy
            .validate_args(&["--allow-hrtime", "script.ts"])
            .is_err()
    );
}

#[test]
fn test_deno_allows_safe_flags() {
    let policy = LauncherPolicy::dev_server(Runtime::Deno);
    assert!(policy.validate_args(&["run", "script.ts"]).is_ok());
    assert!(policy.validate_args(&["task", "dev"]).is_ok());
}

#[test]
fn test_deno_permissions_require_opt_in_without_allowing_eval_flags() {
    let policy = LauncherPolicy {
        runtime: Runtime::Deno,
        allow_dangerous_permissions: true,
    };
    for permission in [
        "--allow-read",
        "--allow-write",
        "--allow-net",
        "--allow-run",
        "--allow-ffi",
        "--allow-hrtime",
    ] {
        assert!(policy.validate_args(&[permission, "script.ts"]).is_ok());
    }
    assert!(policy.validate_args(&["--eval", "console.log(1)"]).is_err());
    assert!(policy.validate_args(&["--allow-all", "script.ts"]).is_err());
}
