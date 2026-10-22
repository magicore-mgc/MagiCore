// Existing v4 policy test cases: mode resolution.
// Test policy V4 hiện có: mode resolution.

#[test]
fn policy_modes_parse_case_insensitively() {
    assert_eq!(LockPolicyMode::parse("off"), Some(LockPolicyMode::Off));
    assert_eq!(LockPolicyMode::parse("WARN"), Some(LockPolicyMode::Warn));
    assert_eq!(
        LockPolicyMode::parse(" Require "),
        Some(LockPolicyMode::Require)
    );
    assert_eq!(LockPolicyMode::parse("sometimes"), None);
}

#[test]
fn ci_policy_cannot_be_downgraded_by_flag_env_or_project_config() {
    let _guard = SERIAL.lock().expect("test operation must succeed");
    let previous_ci = std::env::var_os("CI");
    let previous_policy = std::env::var_os("MGC_LOCK_POLICY");
    unsafe_env_set("CI", "true");
    unsafe_env_set("MGC_LOCK_POLICY", "off");
    let root = tempfile::tempdir().expect("test operation must succeed");
    std::fs::write(root.path().join("mgc.toml"), "[lock]\npolicy = 'off'\n").expect("test operation must succeed");

    let resolved = resolve_policy(Some("off"), Some(root.path()));

    restore_env("CI", previous_ci);
    restore_env("MGC_LOCK_POLICY", previous_policy);
    assert_eq!(resolved, LockPolicyMode::Require);
}

#[cfg(unix)]
#[test]
fn project_lock_policy_ignores_external_config_symlink() {
    let _guard = SERIAL.lock().expect("test operation must succeed");
    let previous_ci = std::env::var_os("CI");
    let previous_policy = std::env::var_os("MGC_LOCK_POLICY");
    unsafe_env_remove("CI");
    unsafe_env_remove("MGC_LOCK_POLICY");
    let root = tempfile::tempdir().expect("test operation must succeed");
    let external = tempfile::tempdir().expect("test operation must succeed");
    std::fs::write(
        external.path().join("mgc.toml"),
        "[lock]\npolicy = 'require'\n",
    )
    .expect("test operation must succeed");
    std::os::unix::fs::symlink(
        external.path().join("mgc.toml"),
        root.path().join("mgc.toml"),
    )
    .expect("test operation must succeed");

    let resolved = resolve_policy(None, Some(root.path()));
    restore_env("CI", previous_ci);
    restore_env("MGC_LOCK_POLICY", previous_policy);
    assert_eq!(resolved, LockPolicyMode::Warn);
}

#[test]
fn flag_beats_everything_in_precedence() {
    // Flag wins even when the environment disagrees — set the env inside
    // a serial guard (see SERIAL below) to avoid cross-test races.
    let _guard = SERIAL.lock().expect("test operation must succeed");
    unsafe_env_set("MGC_LOCK_POLICY", "off");
    assert_eq!(
        resolve_policy(Some("require"), None),
        LockPolicyMode::Require
    );
    unsafe_env_remove("MGC_LOCK_POLICY");
}
