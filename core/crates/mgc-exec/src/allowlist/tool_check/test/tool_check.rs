use crate::allowlist::*;

#[test]
fn allows_known_tools() {
    for tool in [
        "cargo",
        "python",
        "python3",
        "go",
        "mvn",
        "node",
        "git",
        "docker",
        "flutter",
        "terraform",
    ] {
        check_tool(tool).unwrap_or_else(|e| panic!("{tool} should be allowed: {e}"));
    }
}

#[test]
fn bash_is_limited_to_project_execution_scopes() {
    use crate::allowlist::check_tool_with_scope;
    use std::path::Path;

    let root = Path::new("/tmp/project");
    for scope in [
        ExecutionScope::TestRunner,
        ExecutionScope::BuildRunner,
        ExecutionScope::DevServer,
        ExecutionScope::ScriptRunner,
    ] {
        assert!(
            check_tool_with_scope("bash", scope, Some(root)).is_ok(),
            "bash should run project shell code in {scope:?}"
        );
    }
    assert!(
        check_tool_with_scope("bash", ExecutionScope::Install, Some(root)).is_err(),
        "bash must not execute dependency install scripts"
    );
    assert!(
        check_tool_with_scope("bash", ExecutionScope::DeviceControl, Some(root)).is_err(),
        "bash is not a device-control tool"
    );
}

#[test]
fn language_runtimes_are_available_only_in_project_execution_scopes() {
    use crate::allowlist::check_tool_with_scope;
    use std::path::Path;

    let root = Path::new("/tmp/project");
    let tools = [
        "bash", "clang", "clang++", "dotnet", "gcc", "g++", "godot", "java", "javac", "kotlinc",
        "php", "tsx", "zig",
    ];
    for tool in tools {
        for scope in [
            ExecutionScope::TestRunner,
            ExecutionScope::BuildRunner,
            ExecutionScope::DevServer,
            ExecutionScope::ScriptRunner,
        ] {
            assert!(
                check_tool_with_scope(tool, scope, Some(root)).is_ok(),
                "{tool} should be available in {scope:?}"
            );
        }
        for scope in [ExecutionScope::Install, ExecutionScope::DeviceControl] {
            assert!(
                check_tool_with_scope(tool, scope, Some(root)).is_err(),
                "{tool} must stay out of {scope:?}"
            );
        }
    }
}

#[test]
fn tsx_is_limited_to_project_execution_scopes_and_pm_tools_remain_forbidden() {
    use crate::allowlist::check_tool_with_scope;
    use std::path::Path;

    let root = Path::new("/tmp/project");
    for scope in [
        ExecutionScope::TestRunner,
        ExecutionScope::BuildRunner,
        ExecutionScope::DevServer,
        ExecutionScope::ScriptRunner,
    ] {
        assert!(
            check_tool_with_scope("tsx", scope, Some(root)).is_ok(),
            "tsx must run project scripts in {scope:?}"
        );
    }
    for scope in [ExecutionScope::Install, ExecutionScope::DeviceControl] {
        assert!(
            check_tool_with_scope("tsx", scope, Some(root)).is_err(),
            "tsx must not run in {scope:?}"
        );
    }
    for manager in ["npm", "npx", "pnpm", "yarn", "bun"] {
        assert!(
            check_tool_with_scope(manager, ExecutionScope::ScriptRunner, Some(root)).is_err(),
            "package manager {manager} must remain forbidden"
        );
    }
}

#[test]
fn android_debug_bridge_is_allowed_only_for_device_control_scope() {
    use crate::allowlist::check_tool_with_scope;
    use std::path::Path;

    assert!(
        check_tool_with_scope("adb", ExecutionScope::DeviceControl, Some(Path::new("."))).is_ok()
    );
    for scope in [
        ExecutionScope::Install,
        ExecutionScope::TestRunner,
        ExecutionScope::BuildRunner,
        ExecutionScope::DevServer,
    ] {
        assert!(
            check_tool_with_scope("adb", scope, Some(Path::new("."))).is_err(),
            "adb must not be available in {scope:?} scope"
        );
    }
}

#[test]
fn xcrun_simctl_is_allowed_only_for_device_control_scope() {
    use crate::allowlist::check_tool_with_scope;
    use std::path::Path;

    assert!(
        check_tool_with_scope("xcrun", ExecutionScope::DeviceControl, Some(Path::new("."))).is_ok()
    );
    for scope in [
        ExecutionScope::Install,
        ExecutionScope::TestRunner,
        ExecutionScope::BuildRunner,
        ExecutionScope::DevServer,
    ] {
        assert!(
            check_tool_with_scope("xcrun", scope, Some(Path::new("."))).is_err(),
            "xcrun must not be available in {scope:?} scope"
        );
    }
}

#[test]
fn rejects_forbidden_npm_family() {
    for tool in [
        "npm", "npx", "pnpm", "yarn", "bun", "deno", "bunx", "composer", "pub",
    ] {
        let err = check_tool(tool).unwrap_err();
        assert!(
            err.to_string().contains("is forbidden"),
            "npm-family error must be explicit, got: {err}"
        );
    }
}

#[test]
fn rejects_forbidden_absolute_pm_paths() {
    for tool in [
        "/usr/bin/npm",
        "/opt/homebrew/bin/pnpm",
        "C:\\tools\\bun.exe",
    ] {
        let err = check_tool(tool).unwrap_err();
        assert!(
            err.to_string().contains("is forbidden"),
            "absolute PM path must be blocked, got: {err}"
        );
    }
}

#[test]
fn package_managers_are_blocked_in_every_execution_scope_even_with_compat() {
    use crate::allowlist::{check_tool_with_scope, check_tool_with_scope_compat};
    use std::path::Path;

    let root = Path::new("/tmp/project");
    for scope in [
        ExecutionScope::Install,
        ExecutionScope::TestRunner,
        ExecutionScope::BuildRunner,
        ExecutionScope::DevServer,
        ExecutionScope::DeviceControl,
        ExecutionScope::FrameworkInitializer,
    ] {
        for tool in ["npm", "pnpm", "yarn", "bun", "deno", "npx", "bunx"] {
            assert!(
                check_tool_with_scope(tool, scope, Some(root)).is_err(),
                "{tool} unexpectedly allowed in {scope:?}"
            );
        }
        for rival in ["bun", "deno", "composer", "pub"] {
            let err = check_tool_with_scope_compat(rival, scope, Some(root), Some(rival))
                .expect_err("legacy compat mode must never authorize a rival runtime");
            assert!(
                err.to_string().contains("forbidden"),
                "{rival} must be refused by the executor deny-list in {scope:?}: {err}"
            );
        }
    }
}

#[test]
fn allows_absolute_paths_to_allowlisted_tools() {
    check_tool("/usr/bin/cargo").unwrap();
    check_tool("C:\\tools\\python.exe").unwrap();
}

#[test]
fn rejects_unknown_tool() {
    let err = check_tool("totally-made-up-tool").unwrap_err();
    assert!(err.to_string().contains("allowlist"));
}

#[test]
fn rejects_empty_name() {
    assert!(check_tool("").is_err());
    assert!(check_tool("   ").is_err());
}

#[test]
fn project_binary_scope_reuses_device_and_runtime_restrictions() {
    assert!(
        check_project_binary_scope("caller-selected-package-tool", ExecutionScope::Install).is_ok()
    );

    for scope in [
        ExecutionScope::Install,
        ExecutionScope::TestRunner,
        ExecutionScope::BuildRunner,
        ExecutionScope::DevServer,
        ExecutionScope::ScriptRunner,
    ] {
        assert!(check_project_binary_scope("adb", scope).is_err());
        assert!(check_project_binary_scope("xcrun.exe", scope).is_err());
    }
    assert!(check_project_binary_scope("adb.exe", ExecutionScope::DeviceControl).is_ok());
    assert!(check_project_binary_scope("xcrun", ExecutionScope::DeviceControl).is_ok());

    for scope in [ExecutionScope::Install, ExecutionScope::DeviceControl] {
        assert!(check_project_binary_scope("bash", scope).is_err());
    }
    for scope in [
        ExecutionScope::TestRunner,
        ExecutionScope::BuildRunner,
        ExecutionScope::DevServer,
        ExecutionScope::ScriptRunner,
    ] {
        assert!(check_project_binary_scope("bash.exe", scope).is_ok());
    }
}
