use super::*;
use mgc_types::MgError;

#[test]
fn git_name_url_and_repo_name_mapping() {
    assert!(!is_git_name("scope/lib"));
    assert!(is_git_name("github.com/example/repo"));
    assert!(is_git_name("host/x.git"));
    // file:// is NOT a git transport anymore (default-block §13.3):
    // local paths must never reach the clone machinery.
    assert!(!is_git_name("file:///tmp/repo"));
    assert_eq!(
        git_url_to_name("https://github.com/example/repo.git"),
        "github.com/example/repo"
    );
    assert_eq!(
        git_url_to_name("http://github.com/example/repo.git"),
        "http://github.com/example/repo.git"
    );
    assert_eq!(git_repo_name("github.com/example/repo"), "repo");
    assert_eq!(git_repo_name("host/x.git"), "x");
}

#[test]
fn git_transport_is_disabled_even_when_allowlisted() {
    for (url, enabled, hosts) in [
        ("https://github.com/o/r.git", false, &[][..]),
        ("https://github.com/o/r.git", true, &["github.com"][..]),
        ("file:///tmp/repo", true, &["localhost"][..]),
    ] {
        let error =
            git_transport_allowed(url, enabled, hosts).expect_err("test operation must fail");
        match error {
            MgError::Unsupported { guidance, .. } => {
                assert!(guidance.contains("Git transport is disabled"));
                assert!(guidance.contains("external Git executable"));
            }
            other => panic!("expected unsupported transport error, got: {other}"),
        }
    }
}
