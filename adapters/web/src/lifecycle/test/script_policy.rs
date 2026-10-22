//! Lifecycle behavior tests for the matching production owner.
//! Test hành vi lifecycle cho owner production tương ứng.

use super::super::LifecycleRunner;
use super::helpers::write_package_script;

#[test]
fn lifecycle_rejects_external_package_manager_wrappers() {
    let _environment = super::helpers::environment_test_guard();
    let project = tempfile::tempdir().unwrap();
    let package = tempfile::tempdir().unwrap();
    std::fs::write(
        package.path().join("package.json"),
        r#"{"scripts":{"postinstall":"npm run postinstall:inner"}}"#,
    )
    .unwrap();

    let err = LifecycleRunner::run_scripts(package.path(), project.path()).unwrap_err();
    assert!(err.to_string().contains("delegates to 'npm'"));
}

#[test]
fn lifecycle_rejects_pm_wrappers_after_shell_separators() {
    let _environment = super::helpers::environment_test_guard();
    let project = tempfile::tempdir().unwrap();
    let package = tempfile::tempdir().unwrap();
    std::fs::write(
        package.path().join("package.json"),
        r#"{"scripts":{"postinstall":"node build.js && /usr/bin/pnpm install"}}"#,
    )
    .unwrap();

    let err = LifecycleRunner::run_scripts(package.path(), project.path()).unwrap_err();
    assert!(err.to_string().contains("delegates to 'pnpm'"));
}

#[test]
fn lifecycle_rejects_package_manager_javascript_entrypoint() {
    let _environment = super::helpers::environment_test_guard();
    let project = tempfile::tempdir().unwrap();
    let package = tempfile::tempdir().unwrap();
    write_package_script(
        package.path(),
        "node ./node_modules/pnpm/bin/pnpm.cjs install",
    );

    let error = LifecycleRunner::run_scripts(package.path(), project.path()).unwrap_err();
    assert!(error.to_string().contains("delegates to 'pnpm'"));
}

#[test]
fn lifecycle_rejects_shell_control_tokens() {
    let _environment = super::helpers::environment_test_guard();
    let project = tempfile::tempdir().unwrap();
    let package = tempfile::tempdir().unwrap();
    std::fs::write(
        package.path().join("package.json"),
        r#"{"scripts":{"postinstall":"node build.js; node post.js"}}"#,
    )
    .unwrap();

    let err = LifecycleRunner::run_scripts(package.path(), project.path()).unwrap_err();
    assert!(err.to_string().contains("unsupported lifecycle script"));
}

#[test]
fn lifecycle_rejects_rival_runtime_supply_chain_spawn_f_a() {
    let _environment = super::helpers::environment_test_guard();
    // F-A (2026-09-10 supply-chain audit): a dependency package's
    // postinstall MUST NOT be able to spawn deno (previously deno sat on
    // ALLOWED_TOOLS → arbitrary code exec during install). bun was already
    // blocked as a PM; deno is the newly closed hole.
    // Postinstall của dependency KHÔNG được spawn deno (trước đây deno
    // nằm trong ALLOWED_TOOLS → thực thi code tùy ý lúc install).
    for script in ["deno run evil.ts", "bun run evil.ts"] {
        let project = tempfile::tempdir().unwrap();
        let package = tempfile::tempdir().unwrap();
        write_package_script(package.path(), script);

        let err = LifecycleRunner::run_scripts(package.path(), project.path()).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("forbidden")
                || msg.contains("supply-chain guard")
                || msg.contains("permanently forbidden")
                || msg.contains("refuses package-manager wrappers"),
            "lifecycle must refuse rival runtime spawn '{script}': {msg}"
        );
    }
}
