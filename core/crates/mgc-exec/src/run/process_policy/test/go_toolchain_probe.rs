//! Existing exact Go toolchain probe contract.
//! Contract probe toolchain Go chính xác hiện có.

#[test]
fn go_toolchain_version_probe_does_not_bypass_dependency_guards() {
    use std::ffi::OsString;

    for tool in [
        "asm",
        "cgo",
        "compile",
        "cover",
        "link",
        "preprofile",
        "vet",
    ] {
        let args = ["go", "tool", tool, "-V=full"]
            .into_iter()
            .map(OsString::from)
            .collect::<Vec<_>>();
        assert!(
            super::forbidden_process_match_argv("go", &args, &[]).is_none(),
            "Go's exact built-in tool version probe must pass: {args:?}"
        );
    }

    assert!(super::is_go_toolchain_version_probe(&[
        "tool".into(),
        "compile".into(),
        "-v=full".into(),
    ]));
    for args in [
        vec!["tool", "compile", "-v=full", "extra"],
        vec!["tool", "buildid", "-v=full"],
        vec!["tool", "pprof", "-v=full"],
    ] {
        assert!(
            !super::is_go_toolchain_version_probe(
                &args
                    .iter()
                    .map(|arg| (*arg).to_string())
                    .collect::<Vec<_>>()
            ),
            "only the exact supported tool-version query is recognized: {args:?}"
        );
    }

    for args in [
        vec!["go", "mod", "download"],
        vec!["go", "work", "sync"],
        vec!["go", "get", "example.com/pkg"],
        vec!["go", "install", "example.com/pkg"],
        vec!["go", "list", "example.com/pkg"],
        vec!["go", "test", "./..."],
        vec!["go", "run", "main.go"],
    ] {
        let argv = args.iter().map(OsString::from).collect::<Vec<_>>();
        assert!(
            super::forbidden_process_match_argv("go", &argv, &[]).is_some(),
            "non-probe Go invocation must stay blocked without offline proof: {args:?}"
        );
    }

    assert!(
        super::reject_external_dependency_resolution(
            "go",
            &["tool".into(), "compile".into(), "-V=full".into()],
            &[]
        )
        .is_ok(),
        "the shared dependency classifier must recognize the exact read-only probe"
    );
}
