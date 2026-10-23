// policy_tests — split from unit_tests.rs by behavior/subject (P07).
// policy_tests — tách từ unit_tests.rs theo behavior/chủ đề (P07).
use super::*;

#[test]
fn test_lifecycle_scripts_are_opt_in() {
    let old = std::env::var_os("MAGICORE_WEB_ALLOW_SCRIPTS");
    // SAFETY: test-only env mutation; callers hold `env_lock` so no other
    // test thread observes a partially written environment.
    // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
    // không thấy môi trường ghi dở.
    // SAFETY: test-only operation on process-local state / FFI with
    // caller-owned buffers kept alive for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
    // buffer do caller sở hữu và còn sống trong lời gọi.
    unsafe { std::env::remove_var("MAGICORE_WEB_ALLOW_SCRIPTS") };
    assert!(!should_run_lifecycle_scripts(false, false));
    assert!(should_run_lifecycle_scripts(false, true));

    // SAFETY: test-only env mutation; callers hold `env_lock` so no other
    // test thread observes a partially written environment.
    // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
    // không thấy môi trường ghi dở.
    // SAFETY: test-only operation on process-local state / FFI with
    // caller-owned buffers kept alive for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
    // buffer do caller sở hữu và còn sống trong lời gọi.
    unsafe { std::env::set_var("MAGICORE_WEB_ALLOW_SCRIPTS", "1") };
    assert!(should_run_lifecycle_scripts(false, false));
    assert!(!should_run_lifecycle_scripts(true, true));
    restore_env_var("MAGICORE_WEB_ALLOW_SCRIPTS", old);
}

#[test]
fn test_trust_gate_fail_closed_with_escape_hatch() {
    // Fail-closed: unlisted → run only under blanket opt-in.
    assert!(!trust_allows_script(None, false));
    assert!(trust_allows_script(None, true));
    // Explicit policy wins both ways.
    assert!(trust_allows_script(Some("approved"), false));
    assert!(!trust_allows_script(Some("denied"), true));
    // Unknown policy value is treated as unlisted (fail-closed).
    assert!(!trust_allows_script(Some("suspicious"), false));
}

#[test]
fn test_prefetch_defaults_are_conservative() {
    let _guard = env_test_lock().lock().unwrap();
    let old_resolve = std::env::var_os("MAGICORE_WEB_RESOLVE_PREFETCH");
    // SAFETY: test-only env mutation; callers hold `env_lock` so no other
    // test thread observes a partially written environment.
    // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
    // không thấy môi trường ghi dở.
    // SAFETY: test-only operation on process-local state / FFI with
    // caller-owned buffers kept alive for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
    // buffer do caller sở hữu và còn sống trong lời gọi.
    unsafe { std::env::remove_var("MAGICORE_WEB_RESOLVE_PREFETCH") };

    assert!(!resolve_prefetch_enabled());

    restore_env_var("MAGICORE_WEB_RESOLVE_PREFETCH", old_resolve);
}

#[test]
fn test_prefetch_flag_can_be_enabled_explicitly() {
    let _guard = env_test_lock().lock().unwrap();
    let old_resolve = std::env::var_os("MAGICORE_WEB_RESOLVE_PREFETCH");
    // SAFETY: test-only env mutation; callers hold `env_lock` so no other
    // test thread observes a partially written environment.
    // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
    // không thấy môi trường ghi dở.
    // SAFETY: test-only operation on process-local state / FFI with
    // caller-owned buffers kept alive for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
    // buffer do caller sở hữu và còn sống trong lời gọi.
    unsafe { std::env::set_var("MAGICORE_WEB_RESOLVE_PREFETCH", "1") };

    assert!(resolve_prefetch_enabled());

    restore_env_var("MAGICORE_WEB_RESOLVE_PREFETCH", old_resolve);
}

/// P0/F6-RED: age policy loads PER PROJECT (not process-cwd-global) and
/// broken config fails closed — never a swallowed `.ok()`.
/// Policy tuổi nạp theo project, config hỏng fail-closed.
#[tokio::test]
async fn test_age_policy_loader_is_per_project_and_fail_closed() {
    use crate::provider::AgePolicy;
    let good = tempfile::tempdir().unwrap();
    std::fs::write(
        good.path().join("mgc.toml"),
        "name = \"g\"\n[security]\nmin_release_age = 100\ncloud = 7\n",
    )
    .unwrap();
    let policy = crate::WebAdapter::load_age_policy_for(good.path()).unwrap();
    assert_eq!(
        policy,
        Some(AgePolicy {
            cutoff_hours: 100,
            allow_missing_time: false
        }),
        "project policy must load with its own cutoff"
    );
    let cloud_policy = crate::WebAdapter::load_age_policy_for_core(good.path(), "clo").unwrap();
    assert_eq!(
        cloud_policy,
        Some(AgePolicy {
            cutoff_hours: 7,
            allow_missing_time: false
        })
    );
    let web = crate::WebAdapter::new().unwrap();
    web.arm_age_gate_for_core(good.path(), "clo").unwrap();
    assert_eq!(web.provider.age_policy(), cloud_policy);

    // Broken TOML → Err (fail-closed), not silent None.
    let broken = tempfile::tempdir().unwrap();
    std::fs::write(
        broken.path().join("mgc.toml"),
        "[security\nmin_release_age = \n",
    )
    .unwrap();
    assert!(
        crate::WebAdapter::load_age_policy_for(broken.path()).is_err(),
        "broken mgc.toml must fail closed"
    );

    // Wrong-typed field → Err, not silent None.
    let wrongtype = tempfile::tempdir().unwrap();
    std::fs::write(
        wrongtype.path().join("mgc.toml"),
        "name = \"w\"\n[security]\nmin_release_age = \"tomorrow\"\n",
    )
    .unwrap();
    assert!(
        crate::WebAdapter::load_age_policy_for(wrongtype.path()).is_err(),
        "wrong-typed min_release_age must fail closed"
    );

    // Missing file → Ok(None) (historical no-filtering behavior).
    let bare = tempfile::tempdir().unwrap();
    assert!(
        crate::WebAdapter::load_age_policy_for(bare.path())
            .unwrap()
            .is_none()
    );
}
