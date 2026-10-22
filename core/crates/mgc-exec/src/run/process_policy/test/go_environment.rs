//! Existing observed Go environment policy contract.
//! Contract policy env Go quan sát được hiện có.

#[test]
fn observed_go_proxy_is_required_for_readonly_execution_but_never_allows_mutation() {
    use std::ffi::OsString;
    let argv = |args: &[&str]| args.iter().map(OsString::from).collect::<Vec<_>>();
    let off = vec![("GOPROXY".to_string(), "off".to_string())];
    let online = [("GOPROXY".to_string(), "https://proxy.example".to_string())];
    for env in [&[][..], &online[..]] {
        assert!(
            super::forbidden_process_match_argv_with_env(
                "go",
                &argv(&["go", "run", "-mod=readonly", "main.go"]),
                &[],
                env
            )
            .is_some()
        );
    }
    assert!(
        super::forbidden_process_match_argv_with_env(
            "go",
            &argv(&["go", "run", "-mod=readonly", "main.go"]),
            &[],
            &off
        )
        .is_none()
    );
    for args in [
        vec!["go", "get", "example.com/pkg"],
        vec!["go", "run", "main.go"],
        vec!["go", "mod", "download"],
    ] {
        assert!(
            super::forbidden_process_match_argv_with_env("go", &argv(&args), &[], &off).is_some()
        );
    }
    assert!(
        super::forbidden_process_match_argv_with_env("npm", &argv(&["npm", "install"]), &[], &off)
            .is_some()
    );
}
