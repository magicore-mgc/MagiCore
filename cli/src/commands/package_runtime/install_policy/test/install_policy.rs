//! Focused install policy regression tests.
//! Test hồi quy tập trung cho install policy.

use super::*;

#[test]
fn lifecycle_scripts_are_opt_in_for_dlx_installs() {
    assert_eq!(
        registry_install_args(false),
        ["install", "--core", "web", "--ignore-scripts"]
    );
    assert_eq!(
        registry_install_args(true),
        ["install", "--core", "web", "--allow-scripts"]
    );
}
#[test]
fn owned_project_inherits_install_policy_without_execution_compat() {
    use mgc_config::project::{
        CompatConfig, LockConfig, ProjectConfig, SecurityConfig, TrustConfig,
    };
    let root = tempfile::tempdir().expect("isolated caller");
    let mut caller = ProjectConfig::new("caller", "lib");
    caller.security = Some(SecurityConfig {
        min_release_age: Some(876000),
        ..Default::default()
    });
    caller.lock = Some(LockConfig {
        schema: Some("v4".into()),
        policy: Some("require".into()),
        ..Default::default()
    });
    caller.trust = Some(TrustConfig {
        keys: vec!["0123456789abcdef".into()],
    });
    caller.registries.push(mgc_config::registry::Registry::new(
        "private".into(),
        "https://registry.example/team/".into(),
    ));
    caller.compat = Some(CompatConfig {
        allow_git_deps: true,
        ..Default::default()
    });
    std::fs::write(
        root.path().join("mgc.toml"),
        toml::to_string(&caller).expect("serialize caller"),
    )
    .expect("write caller config");
    let owned = registry_project_config(root.path()).expect("inherit caller policy");
    assert_eq!(owned.name, PACKAGE_RUNNER_PROJECT_NAME);
    assert_eq!(owned.version, PACKAGE_RUNNER_PROJECT_VERSION);
    assert_eq!(owned.ecosystem, "web");
    assert_eq!(owned.security, caller.security);
    assert_eq!(owned.lock, caller.lock);
    assert_eq!(owned.trust, caller.trust);
    assert_eq!(owned.registries.len(), 1);
    assert_eq!(owned.registries[0].url, "https://registry.example/team/");
    assert!(owned.compat.is_none());
    let fresh = tempfile::tempdir().expect("owned project");
    std::fs::write(
        fresh.path().join("mgc.toml"),
        toml::to_string(&owned).expect("serialize owned config"),
    )
    .expect("write owned config");
    assert!(mgc_config::project::lock_schema_is_v4(fresh.path()).expect("inherited schema"));
}

#[test]
fn malformed_caller_policy_fails_without_echoing_configuration() {
    let root = tempfile::tempdir().expect("isolated caller");
    std::fs::write(
        root.path().join("mgc.toml"),
        "name = \"fixture-config-value\n",
    )
    .expect("write malformed policy");
    let message = registry_project_config(root.path())
        .expect_err("malformed policy must fail")
        .to_string();
    assert!(message.contains("fix mgc.toml"));
    assert!(!message.contains("fixture-config-value"));
}

#[test]
fn standalone_package_runner_has_native_defaults() {
    let root = tempfile::tempdir().expect("isolated standalone caller");
    let config = registry_project_config(root.path()).expect("standalone defaults");
    assert_eq!(config.ecosystem, "web");
    assert!(config.registries.is_empty());
    assert!(config.security.is_none());
    assert!(config.compat.is_none());
}
