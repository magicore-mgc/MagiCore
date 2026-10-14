#[test]
fn native_astro_dev_disables_external_update_check_only() {
    use crate::commands::compat::CompatMode;
    use std::path::Path;

    assert_eq!(
        super::native_astro_update_check_env(
            Path::new("node_modules/.bin/astro"),
            &CompatMode::Native
        ),
        Some(("ASTRO_DISABLE_UPDATE_CHECK".to_string(), "true".to_string()))
    );
    assert_eq!(
        super::native_astro_update_check_env(
            Path::new("node_modules/.bin/astro.cmd"),
            &CompatMode::Native
        ),
        Some(("ASTRO_DISABLE_UPDATE_CHECK".to_string(), "true".to_string()))
    );
    assert_eq!(
        super::native_astro_update_check_env(
            Path::new("node_modules/.bin/astro"),
            &CompatMode::Explicit("bun".to_string())
        ),
        None
    );
    assert_eq!(
        super::native_astro_update_check_env(
            Path::new("node_modules/.bin/next"),
            &CompatMode::Native
        ),
        None
    );
}

#[test]
fn native_fastapi_dev_uses_uvicorn_asgi_entrypoint() {
    let root = tempfile::tempdir().expect("create FastAPI project");
    std::fs::write(
        root.path().join("mgc.toml"),
        "name = \"fastapi-probe\"\necosystem = \"web\"\nmode = \"backend\"\nframeworks = [\"python\", \"fastapi\"]\n",
    )
    .expect("write project config");
    std::fs::write(
        root.path().join("pyproject.toml"),
        "[project]\nname = \"fastapi-probe\"\nversion = \"0.1.0\"\ndependencies = [\"fastapi>=0.110.0\", \"uvicorn>=0.28.0\"]\n",
    )
    .expect("write Python manifest");
    std::fs::write(root.path().join("main.py"), "app = object()\n").expect("write ASGI entrypoint");

    let launch =
        super::infer_native_dev_launch(root.path(), Some("127.0.0.1".to_string()), Some(4567))
            .expect("infer FastAPI dev launch");

    assert!(
        launch.program.file_name().is_some_and(|name| {
            let name = name.to_string_lossy();
            name == "python" || name.starts_with("python3")
        }),
        "FastAPI must run through the project's Python interpreter: {}",
        launch.program.display()
    );
    assert_eq!(
        launch.args,
        [
            "-m",
            "uvicorn",
            "main:app",
            "--host",
            "127.0.0.1",
            "--port",
            "4567"
        ]
        .into_iter()
        .map(std::ffi::OsString::from)
        .collect::<Vec<_>>()
    );
}

#[test]
fn native_python_dev_keeps_plain_main_script_semantics_without_fastapi_config() {
    let root = tempfile::tempdir().expect("create Python project");
    std::fs::write(root.path().join("main.py"), "print('plain python app')\n")
        .expect("write Python entrypoint");

    let launch = super::infer_native_dev_launch(root.path(), None, None)
        .expect("infer plain Python dev launch");

    assert_eq!(launch.args, [std::ffi::OsString::from("main.py")]);
}

#[test]
fn native_fastapi_dev_supports_src_main_module_layout() {
    let root = tempfile::tempdir().expect("create FastAPI project");
    std::fs::write(
        root.path().join("mgc.toml"),
        "name = \"fastapi-probe\"\necosystem = \"web\"\nframeworks = [\"fastapi\"]\n",
    )
    .expect("write project config");
    std::fs::create_dir(root.path().join("src")).expect("create source directory");
    std::fs::write(root.path().join("src/main.py"), "app = object()\n")
        .expect("write ASGI entrypoint");

    let launch = super::infer_native_dev_launch(root.path(), None, Some(4568))
        .expect("infer FastAPI source-layout dev launch");

    assert_eq!(
        launch.args,
        [
            "-m",
            "uvicorn",
            "src.main:app",
            "--host",
            "localhost",
            "--port",
            "4568"
        ]
        .into_iter()
        .map(std::ffi::OsString::from)
        .collect::<Vec<_>>()
    );
}

#[test]
fn non_python_backend_does_not_parse_python_framework_config() {
    let root = tempfile::tempdir().expect("create Go project");
    std::fs::write(root.path().join("mgc.toml"), "this is not valid TOML = [\n")
        .expect("write malformed unrelated config");
    std::fs::write(root.path().join("go.mod"), "module example.com/server\n")
        .expect("write Go manifest");

    let launch = super::infer_native_dev_launch(root.path(), None, None)
        .expect("Go dev route should not read the FastAPI config");

    assert_eq!(launch.program, std::path::PathBuf::from("go"));
    assert_eq!(launch.args, ["run", "."].map(std::ffi::OsString::from));
}

#[test]
fn python_interpreter_name_detection_covers_supported_executable_names() {
    for name in ["python", "python3", "python3.12", "python312", "python.exe"] {
        assert!(
            super::is_python_interpreter_name(name),
            "expected {name} to be recognized as Python"
        );
    }
    for name in [
        "pythonw",
        "pythonfmt",
        "python3-dev",
        "python3..12",
        "python3.",
        "uvicorn",
        "python-dev",
    ] {
        assert!(
            !super::is_python_interpreter_name(name),
            "expected {name} to stay outside Python interpreter detection"
        );
    }
}

#[test]
fn v3_web_lockfile_is_written_with_current_schema_version() {
    // Deliberate v3 bump (Phase 1): web writes target the v3 schema.
    // Nâng lên v3 có chủ đích (Phase 1): web ghi ra nhắm schema v3.
    let lock = mgc_lockfile::Lockfile::new();
    let encoded = mgc_lockfile::serialization::to_toml(&lock).unwrap();
    assert!(encoded.contains("version = \"3\""));
    let decoded: mgc_lockfile::Lockfile = mgc_lockfile::serialization::from_toml(&encoded).unwrap();
    assert_eq!(decoded.version, "3");
}

#[test]
fn web_framework_aliases_resolve_to_supported_defaults() {
    for (alias, canonical) in [
        ("ts", "vanilla"),
        ("frontend", "vanilla"),
        ("custom", "vanilla"),
        ("node", "express"),
        ("backend", "express"),
        ("fullstack", "remix"),
        ("go", "gin"),
        ("java", "spring-boot"),
        ("php", "laravel"),
        ("python", "fastapi"),
        ("rust", "axum"),
        ("dotnet", "dotnet-minimal"),
        ("monorepo", "react-vite"),
    ] {
        assert_eq!(
            super::parse_framework_request(alias).normalized,
            canonical,
            "alias {alias}"
        );
    }
}

#[test]
fn nuxt_scaffold_seeds_unplugin_at_project_root_for_builder_resolution() {
    let seed = super::FRAMEWORK_SEEDS
        .iter()
        .find(|seed| seed.name == "nuxt")
        .expect("Nuxt framework seed should exist");

    assert!(seed.primary.contains(&"nuxt"));
    assert!(seed.supplemental.contains(&"vue"));
    assert!(seed.toolchain.iter().any(|package| {
        package.section == "devDependencies"
            && package.package == "unplugin"
            && package.version.is_none()
    }));
    assert_eq!(
        super::versions::scaffold_baseline_version("unplugin"),
        Some("^3.0.0")
    );
}

#[test]
fn monorepo_alias_selects_a_frontend_and_backend() {
    let mut flags = crate::commands::core::scaffold_flags::ScaffoldFlags::default();
    super::apply_framework_alias_defaults("monorepo", &mut flags);
    let config = super::build_web_config("monorepo", "sample", &flags).unwrap();
    assert_eq!(config.sub_type, "monorepo");
    assert_eq!(config.frameworks, ["react-vite", "express"]);
}

#[test]
fn monorepo_alias_has_an_offline_frontend_and_backend_scaffold() {
    let root = tempfile::tempdir().unwrap();
    let project = root.path().join("sample-monorepo");
    let mut flags = crate::commands::core::scaffold_flags::ScaffoldFlags::default();
    super::apply_framework_alias_defaults("monorepo", &mut flags);
    let config = super::build_web_config("monorepo", project.to_str().unwrap(), &flags).unwrap();
    let out = crate::scaffold::processor::Scaffolder::scaffold(&config).unwrap();
    assert!(out.join("magicore.workspace.toml").is_file());
    assert!(out.join("apps/frontend/src/App.tsx").is_file());
    assert!(out.join("apps/backend/src/server.js").is_file());
    let package: serde_json::Value =
        serde_json::from_slice(&std::fs::read(out.join("package.json")).unwrap()).unwrap();
    assert_eq!(package["workspaces"][0], "apps/frontend");
    assert_eq!(package["workspaces"][1], "apps/backend");
}

#[test]
fn primary_package_seed_does_not_duplicate_existing_dependency_sections() {
    let package = "@sveltejs/kit";
    let mut manifest = serde_json::Map::new();
    manifest.insert(
        "devDependencies".to_string(),
        serde_json::json!({ package: "^2.0.0" }),
    );
    manifest.insert(
        "dependencies".to_string(),
        serde_json::json!({ "react": "^19.0.0" }),
    );

    super::ensure_primary_package(&mut manifest, package, "^2.69.2");

    let declarations = ["dependencies", "devDependencies"]
        .into_iter()
        .filter(|section| {
            manifest
                .get(*section)
                .and_then(serde_json::Value::as_object)
                .is_some_and(|packages| packages.contains_key(package))
        })
        .count();
    assert_eq!(declarations, 1, "primary package should be declared once");
    assert_eq!(
        manifest["devDependencies"][package], "^2.69.2",
        "the resolved range should update the existing declaration"
    );
    assert!(
        !manifest["dependencies"]
            .as_object()
            .expect("dependencies should remain an object")
            .contains_key(package)
    );
    assert_eq!(manifest["dependencies"]["react"], "^19.0.0");
}

#[test]
fn primary_package_seed_removes_duplicate_from_secondary_section() {
    let package = "@sveltejs/kit";
    let mut manifest = serde_json::json!({
        "dependencies": { package: "^3.0.0" },
        "devDependencies": { package: "^2.0.0" }
    })
    .as_object()
    .expect("manifest should be an object")
    .clone();

    super::ensure_primary_package(&mut manifest, package, "^2.69.2");

    assert_eq!(manifest["dependencies"][package], "^2.69.2");
    assert!(
        !manifest["devDependencies"]
            .as_object()
            .expect("devDependencies should remain an object")
            .contains_key(package)
    );
}

#[test]
fn primary_package_seed_defaults_to_dependencies_when_not_declared() {
    let package = "@sveltejs/kit";
    let mut manifest = serde_json::Map::new();

    super::ensure_primary_package(&mut manifest, package, "^2.69.2");

    assert_eq!(manifest["dependencies"][package], "^2.69.2");
}

#[test]
fn web_preset_only_seeds_implicit_tailwind_without_the_feature_flag() {
    let mut flags = crate::commands::core::scaffold_flags::ScaffoldFlags::default();
    assert!(!super::ui_preset_needs_implicit_tailwind(&flags));

    flags.tailwindcss = true;
    flags.shadcn = true;
    assert!(!super::ui_preset_needs_implicit_tailwind(&flags));

    flags.tailwindcss = false;
    assert!(super::ui_preset_needs_implicit_tailwind(&flags));

    flags.shadcn = false;
    flags.daisyui = true;
    assert!(super::ui_preset_needs_implicit_tailwind(&flags));
}

#[test]
#[cfg(feature = "web")]
fn scaffold_selects_latest_release_that_passes_the_project_age_gate() {
    use mgc_web_adapter::native::npm_registry::{PackageMetadata, VersionInfo};
    use std::collections::HashMap;

    let now = chrono::Utc::now();
    let old_release =
        (now - chrono::Duration::hours(48)).to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let fresh_release =
        (now - chrono::Duration::hours(1)).to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let version_info = |version: &str| VersionInfo {
        version: version.to_string(),
        dependencies: None,
        dev_dependencies: None,
        peer_dependencies: None,
        peer_dependencies_meta: HashMap::new(),
        optional_dependencies: None,
        os: None,
        cpu: None,
        dist: None,
    };
    let metadata = PackageMetadata {
        name: "react-dom".to_string(),
        description: None,
        versions: HashMap::from([
            ("19.3.0".to_string(), version_info("19.3.0")),
            ("19.3.1".to_string(), version_info("19.3.1")),
        ]),
        dist_tags: HashMap::from([("latest".to_string(), "19.3.1".to_string())]),
        time: HashMap::from([
            ("19.3.0".to_string(), old_release.clone()),
            ("19.3.1".to_string(), fresh_release),
        ]),
    };

    let age_policy = super::versions::ScaffoldAgePolicy {
        cutoff_hours: 24,
        allow_missing_time: false,
    };
    assert!(
        super::versions::validate_seed_range_age_eligible(
            "react-dom",
            &metadata,
            age_policy,
            "^19.3.0",
        )
        .is_ok()
    );
    assert!(
        super::versions::validate_seed_range_age_eligible(
            "react-dom",
            &metadata,
            age_policy,
            "^19.3.1",
        )
        .is_err()
    );

    let selected = super::versions::select_latest_eligible_version(
        "react-dom",
        &metadata,
        Some(super::versions::ScaffoldAgePolicy {
            cutoff_hours: 24,
            allow_missing_time: false,
        }),
    )
    .unwrap();

    assert_eq!(selected, "^19.3.0");

    let react_metadata = PackageMetadata {
        name: "react".to_string(),
        description: None,
        versions: HashMap::from([("19.3.1".to_string(), version_info("19.3.1"))]),
        dist_tags: HashMap::from([("latest".to_string(), "19.3.1".to_string())]),
        time: HashMap::from([("19.3.1".to_string(), old_release)]),
    };
    let selected_react = super::versions::select_latest_eligible_version(
        "react",
        &react_metadata,
        Some(super::versions::ScaffoldAgePolicy {
            cutoff_hours: 24,
            allow_missing_time: false,
        }),
    )
    .unwrap();

    assert_eq!(selected_react, "^19.3.1");
}

#[test]
#[cfg(feature = "web")]
fn scaffold_age_gate_fails_closed_when_every_release_is_too_new() {
    use mgc_web_adapter::native::npm_registry::{PackageMetadata, VersionInfo};
    use std::collections::HashMap;

    let fresh_release = (chrono::Utc::now() - chrono::Duration::hours(1))
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let metadata = PackageMetadata {
        name: "react-dom".to_string(),
        description: None,
        versions: HashMap::from([(
            "19.3.1".to_string(),
            VersionInfo {
                version: "19.3.1".to_string(),
                dependencies: None,
                dev_dependencies: None,
                peer_dependencies: None,
                peer_dependencies_meta: HashMap::new(),
                optional_dependencies: None,
                os: None,
                cpu: None,
                dist: None,
            },
        )]),
        dist_tags: HashMap::from([("latest".to_string(), "19.3.1".to_string())]),
        time: HashMap::from([("19.3.1".to_string(), fresh_release)]),
    };

    assert!(
        super::versions::select_latest_eligible_version(
            "react-dom",
            &metadata,
            Some(super::versions::ScaffoldAgePolicy {
                cutoff_hours: 24,
                allow_missing_time: false,
            }),
        )
        .is_err()
    );
}

#[test]
#[cfg(feature = "web")]
fn scaffold_age_gate_does_not_fallback_to_an_unverified_baseline() {
    let error = super::versions::fallback_scaffold_version(
        "react",
        anyhow::anyhow!("registry is offline"),
        Some(super::versions::ScaffoldAgePolicy {
            cutoff_hours: 24,
            allow_missing_time: false,
        }),
    )
    .unwrap_err();

    let message = error.to_string();
    assert!(message.contains("minimum-release-age"));
    assert!(message.contains("react"));
}

#[test]
#[cfg(feature = "web")]
fn scaffold_store_policy_read_failure_is_not_treated_as_no_policy() {
    let root = tempfile::tempdir().unwrap();
    let store_dir = mgc_web_adapter::lockfile::project_cache_dir(root.path());
    std::fs::create_dir_all(&store_dir).unwrap();
    let database_path = store_dir.join("store.db");
    std::fs::write(&database_path, b"not a sqlite database").unwrap();

    assert!(super::versions::store_release_policy(root.path()).is_err());

    std::fs::remove_file(&database_path).unwrap();
    std::fs::create_dir(&database_path).unwrap();
    let error = super::versions::store_release_policy(root.path()).unwrap_err();
    assert!(error.to_string().contains("must be a regular file"));
}

#[test]
#[cfg(feature = "web")]
fn scaffold_age_policy_uses_project_web_override_and_missing_time_escape() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("mgc.toml"),
        "[security]\nweb = 48\nallow_missing_time = true\n",
    )
    .unwrap();

    assert_eq!(
        super::versions::scaffold_age_policy(root.path()).unwrap(),
        Some(super::versions::ScaffoldAgePolicy {
            cutoff_hours: 48,
            allow_missing_time: true,
        })
    );
}

#[cfg(not(feature = "web"))]
#[test]
fn non_web_core_build_does_not_require_the_optional_web_adapter() {
    let root = tempfile::tempdir().unwrap();
    assert!(
        super::versions::scaffold_age_policy(root.path())
            .unwrap()
            .is_none()
    );
}

#[cfg(all(unix, feature = "lib"))]
#[test]
fn backend_language_ignores_external_mgc_toml_symlink() {
    let root = tempfile::tempdir().unwrap();
    let external = tempfile::tempdir().unwrap();
    std::fs::write(
        external.path().join("mgc.toml"),
        "frameworks = ['django']\n",
    )
    .unwrap();
    std::os::unix::fs::symlink(
        external.path().join("mgc.toml"),
        root.path().join("mgc.toml"),
    )
    .unwrap();

    assert_eq!(super::web_backend_language(root.path()), None);
}

/// Vite scripts launch MgDevServer (never the vite binary), so building
/// the launch must NOT require node_modules/.bin/vite to exist — React
/// runs straight through MGC with zero Vite dependence.
/// (Launch vite không cần binary vite — React chạy thẳng qua MGC.)
#[test]
fn vite_dev_launch_needs_no_vite_binary() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        r#"{"name": "x", "scripts": {"dev": "vite --port 4315"}}"#,
    )
    .unwrap();
    assert!(
        !dir.path().join("node_modules").exists(),
        "fixture must have no node_modules"
    );
    let launch = super::build_dev_launch(
        dir.path(),
        "dev",
        None,
        Some(4315),
        &crate::commands::compat::CompatMode::Native,
        None,
    )
    .expect("vite dev launch must build without the binary");
    assert!(
        launch.program.to_string_lossy().ends_with("vite"),
        "vite program routes to MgDevServer downstream"
    );
}

#[test]
fn dev_endpoint_flags_override_values_in_project_scripts() {
    let mut args = [
        "serve",
        "--port=4315",
        "--port",
        "3000",
        "--host=localhost",
        "--host",
        "0.0.0.0",
    ]
    .into_iter()
    .map(std::ffi::OsString::from)
    .collect::<Vec<_>>();

    super::append_dev_endpoint_args(
        &mut args,
        "--host",
        "--port",
        Some("127.0.0.1".to_string()),
        Some(4531),
    );

    assert_eq!(
        args,
        ["serve", "--host", "127.0.0.1", "--port", "4531"]
            .into_iter()
            .map(std::ffi::OsString::from)
            .collect::<Vec<_>>()
    );
}

#[test]
fn dev_endpoint_flags_override_short_options_for_next_scripts() {
    let mut args = ["dev", "-H", "0.0.0.0", "-p", "4315"]
        .into_iter()
        .map(std::ffi::OsString::from)
        .collect::<Vec<_>>();

    super::append_dev_endpoint_args(
        &mut args,
        "-H",
        "-p",
        Some("127.0.0.1".to_string()),
        Some(3451),
    );

    assert_eq!(
        args,
        ["dev", "-H", "127.0.0.1", "-p", "3451"]
            .into_iter()
            .map(std::ffi::OsString::from)
            .collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn multi_dev_supervisor_cancels_and_reaps_sibling_targets() {
    let cancel_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let sibling_stopped = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let mut tasks = tokio::task::JoinSet::<anyhow::Result<()>>::new();

    let sibling_cancel = std::sync::Arc::clone(&cancel_flag);
    let sibling_stopped_flag = std::sync::Arc::clone(&sibling_stopped);
    tasks.spawn(async move {
        while !sibling_cancel.load(std::sync::atomic::Ordering::SeqCst) {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        }
        sibling_stopped_flag.store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    });
    tasks.spawn(async { anyhow::bail!("expected dev target failure") });

    let result = super::supervise_dev_tasks(&mut tasks, cancel_flag).await;

    assert!(result.is_err_and(|error| error.to_string().contains("expected dev target failure")));
    assert!(sibling_stopped.load(std::sync::atomic::Ordering::SeqCst));
    assert!(tasks.is_empty(), "supervisor must reap every dev task");
}

#[test]
fn non_native_monorepo_members_are_classified_without_tool_mapping() {
    // Detect unsupported member ecosystems for accurate fail-closed errors.
    // Nhận diện ecosystem member chưa được hỗ trợ để báo lỗi chính xác.
    use super::non_native_member_ecosystem;
    for (file, tool) in [
        ("go.mod", "go"),
        ("requirements.txt", "python"),
        ("Cargo.toml", "rust"),
        ("pom.xml", "java"),
        ("composer.json", "php"),
        ("artisan", "php"),
    ] {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(file), "x").unwrap();
        assert_eq!(
            non_native_member_ecosystem(dir.path()),
            Some(tool),
            "file {file}"
        );
    }
    let empty = tempfile::tempdir().unwrap();
    assert_eq!(non_native_member_ecosystem(empty.path()), None);
}

#[test]
fn workspace_frontend_path_rejects_layout_escape() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("magicore.workspace.toml"),
        "mode = \"monorepo\"\n[layout]\napps_dir = \"../outside\"\n",
    )
    .unwrap();

    let error = super::workspace_frontend_dir(dir.path())
        .expect_err("web workspace path must stay inside the project root");
    assert!(error.to_string().contains("workspace layout path"));
}

#[cfg(unix)]
#[test]
fn workspace_frontend_path_rejects_symlinked_target() {
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("project");
    let external_frontend = dir.path().join("external-frontend");
    std::fs::create_dir_all(project.join("apps")).unwrap();
    std::fs::create_dir(&external_frontend).unwrap();
    std::os::unix::fs::symlink(external_frontend, project.join("apps/frontend")).unwrap();
    std::fs::write(
        project.join("magicore.workspace.toml"),
        "mode = \"monorepo\"\n[layout]\napps_dir = \"apps\"\n",
    )
    .unwrap();

    let error = super::workspace_frontend_dir(&project)
        .expect_err("web workspace target must not follow a symlink");
    assert!(error.to_string().contains("symlink"));
}

#[test]
fn monorepo_install_rejects_external_package_manager_fallbacks() {
    // No compatibility flag may enable a provider package manager.
    // Không cờ compatibility nào được bật package manager bên ngoài.
    use super::compat_install_target;
    use crate::commands::compat::CompatMode;
    let cargo_member = tempfile::tempdir().unwrap();
    std::fs::write(cargo_member.path().join("Cargo.toml"), "[package]\n").unwrap();
    // Even an exact opt-in is denied and no provider command is run.
    let err = compat_install_target(
        cargo_member.path(),
        &CompatMode::Explicit("cargo".to_string()),
    )
    .expect_err("cargo member with exact compat opt-in must fail closed");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("does not yet own the complete native dependency lifecycle"),
        "must explain native lifecycle is missing: {msg}"
    );
    // Native (no opt-in) → denied.
    compat_install_target(cargo_member.path(), &CompatMode::Native)
        .expect_err("native must fail closed");
    // Unknown member kind → denied even WITH an opt-in.
    let empty = tempfile::tempdir().unwrap();
    compat_install_target(empty.path(), &CompatMode::Explicit("cargo".to_string()))
        .expect_err("unknown member must fail closed");
}

#[test]
fn compat_decision_rejects_every_external_package_manager() {
    // No opt-in may authorize an external package-manager install.
    // Không cờ opt-in nào được cấp quyền cài bằng package manager ngoài.
    use super::compat_gate_decision;
    for manifest in [
        "go.mod",
        "requirements.txt",
        "Cargo.toml",
        "pom.xml",
        "composer.json",
    ] {
        let project = tempfile::tempdir().unwrap();
        std::fs::write(project.path().join(manifest), "fixture").unwrap();
        let error = compat_gate_decision(project.path())
            .expect_err("compat mode must not authorize a provider package manager");
        assert!(format!("{error:#}").contains("native"));
    }
}
