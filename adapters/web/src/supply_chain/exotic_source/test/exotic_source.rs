//! Registry-only policy: paths, Git remotes and URLs are refused.
//! Chính sách chỉ registry: đường dẫn, Git remote và URL bị từ chối.

use super::super::{ensure_registry_only, is_exotic};

#[test]
fn registry_versions_and_ranges_are_allowed() {
    for spec in ["^18.2.0", "1.0.0", "~1.2.3", "*", ">=1 <2", "latest"] {
        assert!(!is_exotic(spec), "'{spec}' stays on the registry");
    }
}

#[test]
fn npm_aliases_are_allowed() {
    // An alias renames a registry package; it does not leave the registry.
    // Alias đổi tên package registry; nó không rời registry.
    assert!(!is_exotic("npm:lodash@^4"));
    assert!(!is_exotic("npm:@scope/pkg@1.2.3"));
}

#[test]
fn paths_git_remotes_and_urls_are_refused() {
    for spec in [
        "file:../secret",
        "file:./vendor",
        "link:../pkg",
        "git+https://github.com/u/r",
        "git+ssh://git@host/r",
        "git://host/r",
        "https://evil.test/pkg.tgz",
        "http://evil.test/pkg.tgz",
        "github:user/repo",
        "gitlab:user/repo",
        "user/repo",
        "user/repo#v1.0.0",
        "workspace:*",
    ] {
        assert!(is_exotic(spec), "'{spec}' must be refused");
    }
}

#[test]
fn scoped_names_are_not_mistaken_for_git_shorthand() {
    // `@scope/name` is one registry package, not `owner/repo`.
    // `@scope/name` là một package registry, không phải `owner/repo`.
    assert!(!is_exotic("@scope/name"));
    assert!(!is_exotic("@scope/name@1.0.0"));
}

#[test]
fn the_refusal_names_every_offending_spec() {
    let deps = vec![
        ("safe".to_string(), "^1.0.0".to_string()),
        ("ptr".to_string(), "file:../secret".to_string()),
        ("url".to_string(), "https://evil.test/x.tgz".to_string()),
    ];
    let error = ensure_registry_only(deps.into_iter()).expect_err("exotic specs are refused");
    let message = error.to_string();
    assert!(message.contains("ptr@file:../secret"), "{message}");
    assert!(message.contains("url@https://evil.test/x.tgz"), "{message}");
    assert!(message.contains("registry only"), "{message}");
}

#[test]
fn a_clean_dependency_set_passes() {
    let deps = vec![
        ("react".to_string(), "^18.2.0".to_string()),
        ("lodash".to_string(), "4.17.21".to_string()),
    ];
    ensure_registry_only(deps.into_iter()).expect("registry specs pass");
}
