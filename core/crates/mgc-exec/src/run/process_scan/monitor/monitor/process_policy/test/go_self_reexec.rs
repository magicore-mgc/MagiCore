use super::super::is_go_build_self_reexec;
use crate::allowlist::ExecutionScope;
use std::ffi::OsString;

#[test]
fn permits_only_readonly_go_build_self_reexec_in_build_runner() {
    let build = [
        OsString::from("/usr/local/go/bin/go"),
        OsString::from("build"),
        OsString::from("-mod=readonly"),
        OsString::from("-o"),
        OsString::from("server"),
        OsString::from("."),
    ];
    assert!(is_go_build_self_reexec(
        "go",
        &build,
        Some("/usr/local/go/bin/go"),
        ExecutionScope::BuildRunner,
    ));
    assert!(!is_go_build_self_reexec(
        "go",
        &build,
        Some("sh"),
        ExecutionScope::BuildRunner,
    ));
    assert!(!is_go_build_self_reexec(
        "go",
        &build,
        Some("go"),
        ExecutionScope::ScriptRunner,
    ));
}

#[test]
fn rejects_go_dependency_commands_and_mutable_module_mode() {
    for args in [
        vec!["go", "get", "example.com/pkg", "-mod=readonly"],
        vec!["go", "list", "-mod=readonly", "./..."],
        vec!["go", "build", "-mod=mod", "."],
        vec!["go", "build", "."],
        vec!["go", "build", "-mod=readonly", "-mod=mod", "."],
        vec!["go", "build", "-mod", "readonly", "."],
    ] {
        let argv = args.into_iter().map(OsString::from).collect::<Vec<_>>();
        assert!(!is_go_build_self_reexec(
            "go",
            &argv,
            Some("go"),
            ExecutionScope::BuildRunner,
        ));
    }
}
