//! Safe diagnostic shape regression contract.
//! Contract hồi quy shape diagnostic an toàn.

#[test]
fn monitor_reports_whether_forbidden_name_matched_image_or_argv() {
    assert_eq!(
        super::forbidden_process_match(
            "node",
            "node /project/node_modules/npm/bin/npm-cli.js dev",
            &[]
        ),
        Some((
            "npm".to_owned(),
            "command-line token",
            Some("npm-cli.js".to_owned()),
            Some(1)
        ))
    );
    assert_eq!(
        super::forbidden_process_match("/usr/local/bin/npm", "npm install", &[]),
        Some(("npm".to_owned(), "executable image", None, None))
    );
    assert_eq!(
        super::matched_invocation_shape("node npm --version", Some(1)),
        "exact read-only probe"
    );
}

#[test]
fn diagnostic_shows_safe_swift_tool_names_but_masks_paths() {
    use std::ffi::OsString;

    let argv = [
        OsString::from("xcrun"),
        OsString::from("swiftc"),
        OsString::from("-show-build"),
        OsString::from("/private/project/build-path"),
    ];
    assert_eq!(
        super::matched_invocation_shape_argv("xcrun", &argv, None),
        "unrecognized invocation; args=[swiftc, -show-build, <arg>]"
    );

    let argv = [
        OsString::from("xcrun"),
        OsString::from(
            "/Applications/Xcode.app/Toolchains/XcodeDefault.xctoolchain/usr/bin/swiftc",
        ),
        OsString::from("-show-build"),
        OsString::from("/private/project/build-path"),
    ];
    assert_eq!(
        super::matched_invocation_shape_argv("xcrun", &argv, None),
        "unrecognized invocation; args=[swiftc, -show-build, <arg>]"
    );

    let argv = [
        OsString::from("xcrun"),
        OsString::from(
            "/Applications/Xcode.app/Toolchains/XcodeDefault.xctoolchain/usr/bin/swift-build-tool",
        ),
        OsString::from("-show-build"),
        OsString::from("/private/project/build-path"),
    ];
    assert_eq!(
        super::matched_invocation_shape_argv("xcrun", &argv, None),
        "unrecognized invocation; args=[swift-build-tool, -show-build, <arg>]"
    );

    let argv = [
        OsString::from("xcrun"),
        OsString::from("--sdk"),
        OsString::from("macosx"),
        OsString::from("--find"),
        OsString::from(
            "/Applications/Xcode.app/Toolchains/XcodeDefault.xctoolchain/usr/bin/swift-inspect",
        ),
    ];
    assert_eq!(
        super::matched_invocation_shape_argv("xcrun", &argv, None),
        "unrecognized invocation; args=[--sdk, <arg>, --find, swift-inspect]"
    );

    let argv = [
        OsString::from("xcrun"),
        OsString::from("/usr/bin/clang"),
        OsString::from("-show-build"),
        OsString::from("/private/project/build-path"),
    ];
    assert_eq!(
        super::matched_invocation_shape_argv("xcrun", &argv, None),
        "unrecognized invocation; args=[clang, -show-build, <arg>]"
    );

    let argv = [
        OsString::from("xcrun"),
        OsString::from("--sdk=/private/sdk/path"),
        OsString::from("-show-build"),
        OsString::from("/private/project/build-path"),
    ];
    assert_eq!(
        super::matched_invocation_shape_argv("xcrun", &argv, None),
        "unrecognized invocation; args=[--sdk=<arg>, -show-build, <arg>]"
    );
}
