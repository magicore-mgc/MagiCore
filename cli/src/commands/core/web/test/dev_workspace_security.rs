// Web command behavior tests — test hành vi command Web.
// Original setups and assertions are retained.

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

    assert_eq!(
        super::super::project_context::adapter_selection::web_backend_language(root.path()),
        None
    );
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
fn startable_server_half_is_skipped_but_a_bare_manifest_is_not() {
    // The install lane skips a half `mgc dev` starts itself. The predicate must
    // require the entry file, so the fail-closed cases below stay untouched.
    // (Lane install bỏ qua nửa do `mgc dev` tự chạy. Predicate phải đòi file
    // entry, để các ca fail-closed bên dưới giữ nguyên.)
    use super::is_startable_server_half;
    for (manifest, entry) in [
        ("Cargo.toml", "src/main.rs"),
        ("go.mod", "main.go"),
        ("pyproject.toml", "main.py"),
        ("pyproject.toml", "app.py"),
        ("pyproject.toml", "manage.py"),
    ] {
        let project = tempfile::tempdir().unwrap();
        std::fs::write(project.path().join(manifest), "fixture").unwrap();
        let entry_path = project.path().join(entry);
        std::fs::create_dir_all(entry_path.parent().unwrap()).unwrap();
        std::fs::write(entry_path, "fixture").unwrap();
        assert!(
            is_startable_server_half(project.path()),
            "{manifest} + {entry} is a startable half"
        );
    }

    // A manifest with nothing to run keeps failing closed.
    // (Manifest không có gì để chạy vẫn fail-closed.)
    for manifest in ["Cargo.toml", "go.mod", "pyproject.toml", "requirements.txt"] {
        let project = tempfile::tempdir().unwrap();
        std::fs::write(project.path().join(manifest), "fixture").unwrap();
        assert!(
            !is_startable_server_half(project.path()),
            "{manifest} alone must not be skipped"
        );
    }

    // A JS project is never a native half, even with an entry-looking file.
    // (Project JS không bao giờ là nửa native, kể cả có file giống entry.)
    let js = tempfile::tempdir().unwrap();
    std::fs::write(js.path().join("package.json"), "{}").unwrap();
    std::fs::write(js.path().join("main.py"), "print('x')").unwrap();
    assert!(!is_startable_server_half(js.path()));
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

#[test]
fn nest_dev_launch_uses_owned_cli_and_application_port_environment() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("node_modules/.bin")).unwrap();
    std::fs::write(dir.path().join("node_modules/.bin/nest"), "fixture").unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        r#"{"scripts":{"dev":"nest start --watch"}}"#,
    )
    .unwrap();
    let launch = super::build_dev_launch(
        dir.path(),
        "dev",
        Some("127.0.0.1".into()),
        Some(4315),
        &crate::commands::compat::CompatMode::Native,
        None,
    )
    .unwrap();
    assert_eq!(launch.program, std::path::PathBuf::from("node"));
    assert!(
        launch
            .args
            .iter()
            .any(|arg| arg == dir.path().join("node_modules/.bin/nest").as_os_str())
    );
    assert_eq!(
        &launch.args[launch.args.len() - 2..],
        &[std::ffi::OsString::from("start"), "--watch".into()]
    );
    assert!(launch.envs.contains(&("PORT".into(), "4315".into())));
    assert!(launch.envs.contains(&("HOST".into(), "127.0.0.1".into())));
}
