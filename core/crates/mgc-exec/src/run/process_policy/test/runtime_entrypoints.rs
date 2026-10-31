//! Existing runtime entrypoint policy contracts.
//! Contract policy entrypoint runtime hiện có.

#[test]
fn runtime_monitor_matches_structured_manager_invocations_without_scanning_code_text() {
    use std::ffi::OsString;

    let args = |values: &[&str]| values.iter().map(OsString::from).collect::<Vec<_>>();

    for (image, argv) in [
        ("sh", args(&["sh", "-c", "echo npm install"])),
        (
            "python3",
            args(&["python3", "-c", "print('npm install; pip install evil')"]),
        ),
        ("node", args(&["node", "-e", "console.log('npm install')"])),
        (
            "node",
            args(&["node", "--require", "/project/npm-cli.js", "app.js"]),
        ),
    ] {
        assert_eq!(
            super::forbidden_process_match_argv(image, &argv, &[]),
            None,
            "opaque script/code text must not be treated as a manager process: {image} {argv:?}"
        );
    }

    let node_args = args(&[
        "node",
        "--require",
        "/project/bootstrap.js",
        "--trace-warnings",
        "/project/node_modules/npm/bin/npm-cli.js",
        "install",
    ]);
    let node_match = super::forbidden_process_match_argv("node", &node_args, &[])
        .expect("Node's npm CLI entrypoint must still be detected");
    assert_eq!(node_match.0, "npm");
    assert_eq!(node_match.1, "runtime entrypoint");
    assert_eq!(node_match.3, Some(4));
    assert!(super::forbidden_process_match_argv("node", &node_args, &["npm"]).is_none());
    assert!(
        crate::run::implementation::process_policy::matched_invocation_shape_argv(
            "npm",
            &node_args,
            node_match.3
        )
        .starts_with("dependency-mutation invocation")
    );

    let python_args = args(&["python3", "-X", "dev", "-m", "pip._internal", "install"]);
    let python_match = super::forbidden_process_match_argv("python3", &python_args, &[])
        .expect("Python -m package manager invocation must be detected");
    assert_eq!(python_match.0, "pip");
    assert_eq!(python_match.1, "runtime entrypoint");
    assert_eq!(python_match.3, Some(4));

    let php_args = args(&["php", "/project/composer.phar", "install"]);
    assert_eq!(
        super::forbidden_process_match_argv("php", &php_args, &[])
            .map(|(name, _, _, _)| name.as_str().to_string()),
        Some("composer".to_string())
    );

    let shell_file_args = args(&["sh", "/project/bin/npm", "install"]);
    assert_eq!(
        super::forbidden_process_match_argv("sh", &shell_file_args, &[])
            .map(|(name, _, _, _)| name),
        Some("npm".to_string()),
        "a shell script whose actual file operand is a manager remains detectable"
    );

    let direct_args = args(&["/tools/npm", "--version"]);
    assert_eq!(
        super::forbidden_process_match_argv("npm", &direct_args, &[]).map(|(name, _, _, _)| name),
        Some("npm".to_string()),
        "a manager executable image remains forbidden regardless of its arguments"
    );

    for command in [
        "npm install",
        "\"C:\\Program Files\\nodejs\\npm.cmd\" install",
        "\"npm install\"",
        "\"\"C:\\Program Files\\nodejs\\npm.cmd\" install\"",
    ] {
        let cmd_args = args(&["cmd.exe", "/d", "/s", "/c", command]);
        assert_eq!(
            super::forbidden_process_match_argv("cmd.exe", &cmd_args, &[])
                .map(|(name, _, _, _)| name),
            Some("npm".to_string()),
            "cmd /c must expose only its first executable: {command}"
        );
    }

    for command in ["echo npm install", "\"echo npm install\""] {
        let cmd_args = args(&["cmd.exe", "/d", "/s", "/c", command]);
        assert_eq!(
            super::forbidden_process_match_argv("cmd.exe", &cmd_args, &[]),
            None,
            "a manager name after cmd's first executable is not an invocation: {command}"
        );
    }

    for (image, argv, expected) in [
        (
            "rustc",
            args(&[
                "rustc",
                "--crate-name",
                "test_lib",
                "--crate-type",
                "lib",
                "/project/src/lib.rs",
            ]),
            None, // Compiler descendants are execution, not dependency mutation.
        ),
        ("cargo", args(&["cargo", "fetch"]), Some("cargo")),
        (
            "go",
            args(&["go", "get", "example.com/acme/lib"]),
            Some("go"),
        ),
        ("flutter", args(&["flutter", "pub", "get"]), Some("pub")),
        (
            "swift",
            args(&["swift", "package", "resolve"]),
            Some("swift"),
        ),
        (
            "dotnet",
            args(&["dotnet", "add", "package", "Example"]),
            Some("dotnet"),
        ),
        ("gradle", args(&["gradle", "build"]), Some("gradle")),
        ("mvn", args(&["mvn", "test"]), Some("mvn")),
        (
            "go",
            args(&["go", "build", "-mod=readonly"]),
            Some("go"), // GOPROXY cannot be verified from a descendant environment.
        ),
        (
            "cargo",
            args(&["cargo", "build", "--locked", "--offline"]),
            None,
        ),
        // Online is the product default now; `--locked` is the only mandatory
        // pin, so a networked `cargo build --locked` must not be denied as if
        // it were resolving dependencies behind MagiCore's back.
        // (`--locked` là pin bắt buộc duy nhất; build online không bị chặn.)
        ("cargo", args(&["cargo", "build", "--locked"]), None),
        ("flutter", args(&["flutter", "run", "--no-pub"]), None),
        ("gradle", args(&["gradle", "build", "--offline"]), None),
        ("mvn", args(&["mvn", "test", "-o"]), None),
    ] {
        let found =
            super::forbidden_process_match_argv(image, &argv, &[]).map(|(name, _, _, _)| name);
        assert_eq!(
            found.as_deref(),
            expected,
            "dependency mutation policy mismatch: {image} {argv:?}"
        );
    }

    for command in [
        "cargo fetch",
        "\"C:\\Program Files\\Rust\\cargo.exe\" fetch",
    ] {
        let cmd_args = args(&["cmd.exe", "/d", "/s", "/c", command]);
        assert_eq!(
            super::forbidden_process_match_argv("cmd.exe", &cmd_args, &[])
                .map(|(name, _, _, _)| name),
            Some("cargo".to_string()),
            "cmd /c must enforce dependency mutation verbs: {command}"
        );
    }

    let shell_cargo = args(&["sh", "/project/bin/cargo.sh", "fetch"]);
    assert_eq!(
        super::forbidden_process_match_argv("sh", &shell_cargo, &[]).map(|(name, _, _, _)| name),
        Some("cargo".to_string()),
        "a script wrapper must not hide a dependency mutation verb"
    );
}
