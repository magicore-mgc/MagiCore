// Focused argv matcher boundary tests.
// Test biên tập trung cho matcher argv.

use super::*;
use std::ffi::OsString;

#[test]
fn direct_image_match_precedes_runtime_parsing() {
    let argv = vec![OsString::from("npm"), OsString::from("install")];
    assert_eq!(
        forbidden_process_match_argv("npm", &argv, &[]).map(|(_, reason, _, _)| reason),
        Some("executable image")
    );
}

#[test]
fn inline_node_source_remains_unclassified() {
    let argv = vec![
        OsString::from("node"),
        OsString::from("-e"),
        OsString::from("npm install"),
    ];
    assert_eq!(forbidden_process_match_argv("node", &argv, &[]), None);
}

#[test]
fn xcrun_read_only_sdk_queries_are_not_device_control_in_a_build() {
    use crate::allowlist::ExecutionScope;
    // Cargo/rustc query the Apple SDK through xcrun during a build; that is a
    // read-only SDK lookup, not device control, so it must not be killed.
    // (Cargo/rustc truy vấn SDK Apple qua xcrun khi build; chỉ đọc, không phải
    // điều khiển thiết bị, nên không bị kill.)
    for query in ["--show-sdk-version", "--show-sdk-path", "--find"] {
        let argv = [OsString::from("xcrun"), OsString::from(query), OsString::from("clang")];
        let blocked = forbidden_process_match_argv_with_scope(
            "xcrun",
            &argv,
            &[],
            &[],
            ExecutionScope::BuildRunner,
        );
        assert_eq!(blocked, None, "xcrun {query} must be allowed during a build");
    }
    // A real device-control invocation (simctl) is still blocked outside
    // DeviceControl scope.
    // (Lệnh điều khiển thiết bị thật (simctl) vẫn bị chặn ngoài DeviceControl.)
    let simctl = [OsString::from("xcrun"), OsString::from("simctl"), OsString::from("list")];
    let blocked = forbidden_process_match_argv_with_scope(
        "xcrun",
        &simctl,
        &[],
        &[],
        ExecutionScope::BuildRunner,
    );
    assert!(blocked.is_some(), "xcrun simctl must stay blocked");
}

#[test]
fn device_control_descendants_follow_the_root_execution_scope() {
    use crate::allowlist::ExecutionScope;

    let adb_argv = [
        OsString::from("/android/platform-tools/adb.exe"),
        OsString::from("devices"),
    ];
    let blocked = forbidden_process_match_argv_with_scope(
        r"C:\Android\platform-tools\adb.exe",
        &adb_argv,
        &[],
        &[],
        ExecutionScope::BuildRunner,
    );
    assert_eq!(
        blocked,
        Some(("adb".to_owned(), "device-control scope", None, None))
    );

    let xcrun_argv = [OsString::from("xcrun"), OsString::from("simctl")];
    let blocked = forbidden_process_match_argv_with_scope(
        "xcrun",
        &xcrun_argv,
        &[],
        &[],
        ExecutionScope::Install,
    );
    assert_eq!(
        blocked.map(|(name, source, _, index)| (name, source, index)),
        Some(("xcrun".to_owned(), "device-control scope", None))
    );

    assert!(forbidden_process_match_argv_with_scope(
        "adb",
        &adb_argv,
        &[],
        &[],
        ExecutionScope::DeviceControl,
    )
    .is_none());
    assert!(forbidden_process_match_argv_with_scope(
        "npm",
        &[OsString::from("npm"), OsString::from("install")],
        &[],
        &[],
        ExecutionScope::DeviceControl,
    )
    .is_some());
}

#[test]
fn framework_initializer_allows_only_exact_read_only_package_manager_probes() {
    use crate::allowlist::ExecutionScope;

    let bun_shim_version = [
        OsString::from("sh"),
        OsString::from("/tmp/initializer/node_modules/.bin/bun"),
        OsString::from("--version"),
    ];
    assert!(forbidden_process_match_argv_with_scope(
        "sh",
        &bun_shim_version,
        &[],
        &[],
        ExecutionScope::FrameworkInitializer,
    )
    .is_none());
    assert!(forbidden_process_match_argv_with_scope(
        "sh",
        &bun_shim_version,
        &[],
        &[],
        ExecutionScope::ScriptRunner,
    )
    .is_some());

    assert!(forbidden_process_match_argv_with_scope(
        "npm",
        &[OsString::from("npm"), OsString::from("install")],
        &[],
        &[],
        ExecutionScope::FrameworkInitializer,
    )
    .is_some());
    let npm_config_query = [
        OsString::from("sh"),
        OsString::from("/tmp/initializer/node_modules/.bin/npm"),
        OsString::from("config"),
        OsString::from("get"),
        OsString::from("registry"),
    ];
    assert!(forbidden_process_match_argv_with_scope(
        "sh",
        &npm_config_query,
        &[],
        &[],
        ExecutionScope::FrameworkInitializer,
    )
    .is_some());
}
