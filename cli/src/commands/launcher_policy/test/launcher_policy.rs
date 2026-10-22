use super::super::{LauncherPolicy, Runtime};

#[test]
fn test_policy_constructors_default_to_disallowing_dangerous_permissions() {
    for runtime in [Runtime::Bun, Runtime::Deno, Runtime::Node] {
        assert!(!LauncherPolicy::dev_server(runtime).allow_dangerous_permissions);
        assert!(!LauncherPolicy::test_runner(runtime).allow_dangerous_permissions);
    }
}
