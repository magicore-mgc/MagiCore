use super::super::is_swift_toolchain_build_probe;
use crate::allowlist::ExecutionScope;
use std::ffi::OsString;

#[test]
fn permits_only_swift_driver_sdk_path_queries_in_project_toolchain_scopes() {
    let no_args = [OsString::from("xcrun")];
    assert!(is_swift_toolchain_build_probe(
        "xcrun",
        &no_args,
        ExecutionScope::DevServer,
    ));
    assert!(!is_swift_toolchain_build_probe(
        "xcrun",
        &no_args,
        ExecutionScope::ScriptRunner,
    ));

    let argv = [
        OsString::from("/usr/bin/xcrun"),
        OsString::from("--sdk"),
        OsString::from("macosx"),
        OsString::from("--show-sdk-platform-path"),
    ];
    assert!(is_swift_toolchain_build_probe(
        "/usr/bin/xcrun",
        &argv,
        ExecutionScope::DevServer,
    ));
    assert!(is_swift_toolchain_build_probe(
        "xcrun",
        &argv,
        ExecutionScope::BuildRunner,
    ));
    assert!(is_swift_toolchain_build_probe(
        "xcrun",
        &argv,
        ExecutionScope::DevServer,
    ));
    assert!(!is_swift_toolchain_build_probe(
        "xcrun",
        &argv,
        ExecutionScope::DeviceControl,
    ));

    let sdk_path = [
        OsString::from("xcrun"),
        OsString::from("--sdk"),
        OsString::from("macosx"),
        OsString::from("--show-sdk-path"),
    ];
    assert!(is_swift_toolchain_build_probe(
        "xcrun",
        &sdk_path,
        ExecutionScope::TestRunner,
    ));
    assert!(!is_swift_toolchain_build_probe(
        "xcrun",
        &sdk_path,
        ExecutionScope::ScriptRunner,
    ));
}

#[test]
fn rejects_device_commands_untrusted_scopes_and_other_xcrun_argv() {
    let simctl = [OsString::from("xcrun"), OsString::from("simctl")];
    assert!(!is_swift_toolchain_build_probe(
        "xcrun",
        &simctl,
        ExecutionScope::DevServer,
    ));

    let wrong_operation = [
        OsString::from("xcrun"),
        OsString::from("--sdk"),
        OsString::from("macosx"),
        OsString::from("-show-build"),
    ];
    assert!(!is_swift_toolchain_build_probe(
        "xcrun",
        &wrong_operation,
        ExecutionScope::DevServer,
    ));

    let user_shell = [
        OsString::from("xcrun"),
        OsString::from("--sdk"),
        OsString::from("macosx"),
        OsString::from("--show-sdk-platform-path"),
    ];
    assert!(!is_swift_toolchain_build_probe(
        "xcrun",
        &user_shell,
        ExecutionScope::ScriptRunner,
    ));

    let extra_args = [
        OsString::from("xcrun"),
        OsString::from("--sdk"),
        OsString::from("macosx"),
        OsString::from("--show-sdk-platform-path"),
        OsString::from("--run"),
    ];
    assert!(!is_swift_toolchain_build_probe(
        "xcrun",
        &extra_args,
        ExecutionScope::DevServer,
    ));
}

#[test]
fn permits_only_swift_sdk_tool_lookup_and_build_info_queries() {
    let sdk_find = [
        OsString::from("xcrun"),
        OsString::from("--sdk"),
        OsString::from("macosx"),
        OsString::from("--find"),
        OsString::from("swiftc"),
    ];
    assert!(is_swift_toolchain_build_probe(
        "xcrun",
        &sdk_find,
        ExecutionScope::DevServer,
    ));

    let tool_find = [
        OsString::from("xcrun"),
        OsString::from("--find"),
        OsString::from("swift-frontend"),
    ];
    assert!(is_swift_toolchain_build_probe(
        "xcrun",
        &tool_find,
        ExecutionScope::DevServer,
    ));

    let xctest_find = [
        OsString::from("xcrun"),
        OsString::from("--sdk"),
        OsString::from("macosx"),
        OsString::from("--find"),
        OsString::from("xctest"),
    ];
    assert!(is_swift_toolchain_build_probe(
        "xcrun",
        &xctest_find,
        ExecutionScope::TestRunner,
    ));

    let toolchain_find = [
        OsString::from("xcrun"),
        OsString::from("-toolchain"),
        OsString::from("com.apple.dt.toolchain.XcodeDefault"),
        OsString::from("-f"),
        OsString::from("swiftc"),
    ];
    assert!(is_swift_toolchain_build_probe(
        "xcrun",
        &toolchain_find,
        ExecutionScope::BuildRunner,
    ));

    let metal_find = [
        OsString::from("xcrun"),
        OsString::from("--find"),
        OsString::from("metal"),
    ];
    assert!(is_swift_toolchain_build_probe(
        "xcrun",
        &metal_find,
        ExecutionScope::BuildRunner,
    ));

    let device_tool_find = [
        OsString::from("xcrun"),
        OsString::from("--find"),
        OsString::from("simctl"),
    ];
    assert!(is_swift_toolchain_build_probe(
        "xcrun",
        &device_tool_find,
        ExecutionScope::DevServer,
    ));

    let swift_build_info = [
        OsString::from("xcrun"),
        OsString::from(
            "/Applications/Xcode.app/Toolchains/XcodeDefault.xctoolchain/usr/bin/swiftc",
        ),
        OsString::from("-show-build"),
        OsString::from(
            "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk",
        ),
    ];
    assert!(is_swift_toolchain_build_probe(
        "xcrun",
        &swift_build_info,
        ExecutionScope::BuildRunner,
    ));

    let vtool_build_info = [
        OsString::from("xcrun"),
        OsString::from("/Applications/Xcode.app/Contents/Developer/usr/bin/vtool"),
        OsString::from("-show-build"),
        OsString::from("/private/project/App"),
    ];
    assert!(is_swift_toolchain_build_probe(
        "xcrun",
        &vtool_build_info,
        ExecutionScope::BuildRunner,
    ));

    let simctl_lookup = [
        OsString::from("xcrun"),
        OsString::from("--sdk"),
        OsString::from("macosx"),
        OsString::from("--find"),
        OsString::from("simctl"),
    ];
    assert!(is_swift_toolchain_build_probe(
        "xcrun",
        &simctl_lookup,
        ExecutionScope::DevServer,
    ));
}
