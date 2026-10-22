use crate::allowlist::compat_runner_allows;

#[test]
fn allows_declared_runner_basename() {
    assert!(compat_runner_allows("npm", Some("npm")));
    assert!(compat_runner_allows(
        "/x/node_modules/.bin/npm",
        Some("npm")
    ));
    assert!(!compat_runner_allows("pip", Some("npm")));
    assert!(!compat_runner_allows("npm", None));
    assert!(!compat_runner_allows("npm", Some("yarn")));
}
