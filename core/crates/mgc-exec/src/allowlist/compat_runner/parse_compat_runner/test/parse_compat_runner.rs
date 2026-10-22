use crate::allowlist::{QUALIFIED_COMPAT_RUNNERS, parse_compat_runner};

#[test]
fn qualified_managers_parse() {
    for manager in ["npm", "pnpm", "yarn", "bun"] {
        assert!(QUALIFIED_COMPAT_RUNNERS.contains(&manager));
        assert_eq!(
            parse_compat_runner(Some(manager)).unwrap(),
            Some(manager.to_string())
        );
    }
    assert_eq!(parse_compat_runner(None).unwrap(), None);
}

#[test]
fn parse_normalizes_manager_name() {
    assert_eq!(
        parse_compat_runner(Some("/usr/local/bin/NPM")).unwrap(),
        Some("npm".to_string())
    );
    assert_eq!(
        parse_compat_runner(Some("Yarn.CMD")).unwrap(),
        Some("yarn".to_string())
    );
}

#[test]
fn unqualified_managers_rejected() {
    for manager in ["pip", "uv", "cargo", "go", "pub", "composer", "deno"] {
        let err = parse_compat_runner(Some(manager)).unwrap_err();
        assert!(
            err.to_string().contains("not a qualified compat manager"),
            "unexpected error for {manager}: {err}"
        );
    }
}
