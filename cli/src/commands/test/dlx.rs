//! Registry request and temporary-workspace containment regressions.
//! Khóa request hợp lệ và chặn traversal/symlink trong môi trường dlx.

use super::*;

#[test]
fn lifecycle_scripts_are_opt_in_for_dlx_installs() {
    assert_eq!(
        dlx_install_args(false),
        ["install", "--core", "web", "--ignore-scripts"]
    );
    assert_eq!(
        dlx_install_args(true),
        ["install", "--core", "web", "--allow-scripts"]
    );
}

#[test]
fn accepts_registry_names_versions_ranges_and_tags() {
    for (request, expected) in [
        ("pkg", ("pkg", "latest")),
        ("@scope/pkg", ("@scope/pkg", "latest")),
        ("pkg@1.2.3", ("pkg", "1.2.3")),
        ("@scope/pkg@^1.2.3", ("@scope/pkg", "^1.2.3")),
        ("pkg@>=1.2.0 <2", ("pkg", ">=1.2.0 <2")),
        ("pkg@beta", ("pkg", "beta")),
    ] {
        assert_eq!(
            parse_request(request).expect("valid registry request"),
            expected
        );
    }
}

#[test]
fn rejects_traversal_urls_controls_and_malformed_names() {
    for request in [
        "",
        "../pkg",
        "/pkg",
        "@scope",
        "@scope/../pkg",
        "@scope/pkg/extra",
        "pkg@",
        "pkg@   ",
        "pkg@../../../../../../escaped",
        "pkg@C:\\escaped",
        "pkg@https://example.invalid/a.tgz",
        "pkg@%2f..",
        "pkg@bad\nvalue",
        "pkg@bad\0value",
        "pkg@@tag",
        "_pkg",
        ".pkg",
    ] {
        assert!(
            parse_request(request).is_err(),
            "unsafe request must be rejected"
        );
    }
    assert!(parse_request(&"a".repeat(MAX_NPM_PACKAGE_NAME_BYTES + 1)).is_err());
    assert!(parse_request(&format!("pkg@{}", "a".repeat(MAX_DLX_REQUEST_BYTES))).is_err());
}

#[test]
fn creates_workspace_under_configured_data_root() {
    let data = tempfile::tempdir().expect("isolated data root");
    let root = work_root_at(data.path()).expect("safe dlx root");
    assert_eq!(
        root,
        data.path()
            .canonicalize()
            .expect("canonical data root")
            .join("cache/web/dlx")
    );
    let project = tempfile::Builder::new()
        .prefix(DLX_WORK_PREFIX)
        .tempdir_in(&root)
        .expect("fresh workspace");
    assert!(project.path().starts_with(&root));
}

#[test]
fn refuses_file_in_cache_directory_chain() {
    let data = tempfile::tempdir().expect("isolated data root");
    std::fs::write(data.path().join("cache"), "not a directory").expect("write obstruction");
    assert!(work_root_at(data.path()).is_err());
}

#[cfg(unix)]
#[test]
fn refuses_symlinks_at_every_cache_directory_level() {
    for position in 0..DLX_WORK_DIRECTORY_COMPONENTS.len() {
        let data = tempfile::tempdir().expect("isolated data root");
        let outside = tempfile::tempdir().expect("isolated outside directory");
        let mut path = data.path().to_path_buf();
        for component in &DLX_WORK_DIRECTORY_COMPONENTS[..position] {
            path.push(component);
            std::fs::create_dir(&path).expect("create real parent");
        }
        path.push(DLX_WORK_DIRECTORY_COMPONENTS[position]);
        std::os::unix::fs::symlink(outside.path(), &path).expect("plant redirected cache");
        assert!(
            work_root_at(data.path()).is_err(),
            "cache symlink must be rejected"
        );
        assert_eq!(
            std::fs::read_dir(outside.path())
                .expect("inspect outside directory")
                .count(),
            0
        );
    }
}

fn entries(rows: &[(&str, &str)]) -> Vec<(String, PathBuf)> {
    rows.iter()
        .map(|(name, target)| ((*name).into(), (*target).into()))
        .collect()
}
#[test]
fn single_nonmatching_name() {
    assert_eq!(
        select_bin_name(&entries(&[("serve", "cli.js")]), "some-package", None).unwrap(),
        "serve"
    );
}
#[test]
fn scoped_basename_wins() {
    assert_eq!(
        select_bin_name(
            &entries(&[("helper", "helper.js"), ("proof", "proof.js")]),
            "@mgc/proof",
            None
        )
        .unwrap(),
        "proof"
    );
}
#[test]
fn explicit_typescript_selection() {
    assert_eq!(
        select_bin_name(
            &entries(&[("tsc", "bin/tsc"), ("tsserver", "bin/tsserver")]),
            "typescript",
            Some("tsc")
        )
        .unwrap(),
        "tsc"
    );
}
#[test]
fn ambiguous_typescript_is_not_guessed() {
    assert!(
        select_bin_name(
            &entries(&[("tsc", "bin/tsc"), ("tsserver", "bin/tsserver")]),
            "typescript",
            None
        )
        .is_err()
    );
}
#[test]
fn aliases_choose_deterministically() {
    assert_eq!(
        select_bin_name(&entries(&[("z", "cli.js"), ("a", "cli.js")]), "pkg", None).unwrap(),
        "a"
    );
}
#[test]
fn undeclared_dependency_bin_and_path_are_denied() {
    for selected in ["other", "../bin", "/bin", "..", "", "bad\nname"] {
        assert!(select_bin_name(&entries(&[("safe", "cli.js")]), "pkg", Some(selected)).is_err());
    }
}
#[test]
fn no_bin_and_unsafe_declared_name_are_denied() {
    assert!(select_bin_name(&[], "pkg", None).is_err());
    assert!(select_bin_name(&entries(&[("../escape", "cli.js")]), "pkg", None).is_err());
}

#[test]
fn requested_package_bin_metadata_is_owned_and_typed() {
    let root = tempfile::tempdir().expect("isolated installed package");
    for (manifest, expected) in [
        (
            serde_json::json!({"name":"@scope/pkg","bin":"cli.js"}),
            vec![("pkg".to_string(), PathBuf::from("cli.js"))],
        ),
        (
            serde_json::json!({"name":"@scope/pkg","bin":{"tool":"bin/tool.js"}}),
            vec![("tool".to_string(), PathBuf::from("bin/tool.js"))],
        ),
        (serde_json::json!({"name":"@scope/pkg"}), vec![]),
    ] {
        std::fs::write(root.path().join("package.json"), manifest.to_string())
            .expect("write installed manifest");
        assert_eq!(
            requested_package_bins(root.path(), "@scope/pkg").expect("owned typed manifest"),
            expected
        );
    }
    for manifest in [
        serde_json::json!({"name":"dependency","bin":"cli.js"}),
        serde_json::json!({"name":"@scope/pkg","bin":{"tool":true}}),
        serde_json::json!({"name":"@scope/pkg","bin":["tool"]}),
    ] {
        std::fs::write(root.path().join("package.json"), manifest.to_string())
            .expect("write invalid manifest");
        assert!(requested_package_bins(root.path(), "@scope/pkg").is_err());
    }
}

#[cfg(unix)]
#[test]
fn requested_package_metadata_symlink_is_rejected() {
    let root = tempfile::tempdir().expect("isolated installed package");
    let foreign = tempfile::NamedTempFile::new().expect("foreign metadata");
    std::fs::write(foreign.path(), r#"{"name":"pkg","bin":"cli.js"}"#)
        .expect("write foreign metadata");
    std::os::unix::fs::symlink(foreign.path(), root.path().join("package.json"))
        .expect("redirect installed metadata");
    assert!(requested_package_bins(root.path(), "pkg").is_err());
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
    let owned = dlx_project_config(root.path()).expect("inherit caller policy");
    assert_eq!(owned.name, DLX_PROJECT_NAME);
    assert_eq!(owned.version, DLX_PROJECT_VERSION);
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
    let message = dlx_project_config(root.path())
        .expect_err("malformed policy must fail")
        .to_string();
    assert!(message.contains("fix mgc.toml"));
    assert!(!message.contains("fixture-config-value"));
}

#[test]
fn standalone_dlx_has_native_defaults() {
    let root = tempfile::tempdir().expect("isolated standalone caller");
    let config = dlx_project_config(root.path()).expect("standalone defaults");
    assert_eq!(config.ecosystem, "web");
    assert!(config.registries.is_empty());
    assert!(config.security.is_none());
    assert!(config.compat.is_none());
}
