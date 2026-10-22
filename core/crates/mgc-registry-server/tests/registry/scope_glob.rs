#[test]
fn scope_glob_matches() {
    use mgc_registry_server::auth::scope_matches;
    assert!(scope_matches("*", "anything"));
    assert!(scope_matches("@magicore/*", "@magicore/core"));
    assert!(scope_matches("@magicore/*", "@magicore/core/extra"));
    assert!(!scope_matches("@magicore/*", "@other/pkg"));
    assert!(scope_matches("mypkg", "mypkg"));
    assert!(!scope_matches("mypkg", "mypkg2"));
    assert!(scope_matches("myapp/*", "myapp"));
}
