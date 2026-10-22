// Web command behavior tests — test hành vi command Web.
// Original setups and assertions are retained.

#[test]
fn native_astro_dev_disables_external_update_check_only() {
    use crate::commands::compat::CompatMode;
    use std::path::Path;

    assert_eq!(
        super::super::dev::native_astro_update_check_env(
            Path::new("node_modules/.bin/astro"),
            &CompatMode::Native
        ),
        Some(("ASTRO_DISABLE_UPDATE_CHECK".to_string(), "true".to_string()))
    );
    assert_eq!(
        super::super::dev::native_astro_update_check_env(
            Path::new("node_modules/.bin/astro.cmd"),
            &CompatMode::Native
        ),
        Some(("ASTRO_DISABLE_UPDATE_CHECK".to_string(), "true".to_string()))
    );
    assert_eq!(
        super::super::dev::native_astro_update_check_env(
            Path::new("node_modules/.bin/astro"),
            &CompatMode::Explicit("bun".to_string())
        ),
        None
    );
    assert_eq!(
        super::super::dev::native_astro_update_check_env(
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
fn native_go_dev_uses_mgc_module_cache_and_disables_network_fallbacks() {
    let root = tempfile::tempdir().expect("create Go store roots");
    let module_cache = root.path().join("store").join("go");
    let build_cache = root.path().join("cache").join("go").join("build");

    let environment =
        super::super::project_context::native_install_environment::go_native_runtime_env(
            &module_cache,
            &build_cache,
        );

    assert!(environment.contains(&("GOMODCACHE".to_string(), module_cache.display().to_string())));
    assert!(environment.contains(&("GOCACHE".to_string(), build_cache.display().to_string())));
    assert!(environment.contains(&("GOPROXY".to_string(), "off".to_string())));
    assert!(environment.contains(&("GOSUMDB".to_string(), "off".to_string())));
    assert!(environment.contains(&("GOTOOLCHAIN".to_string(), "local".to_string())));
}

#[test]
fn native_rust_dev_uses_the_verified_mgc_directory_source_offline() {
    let root = tempfile::tempdir().expect("create managed Cargo source");

    let args = super::cargo_run_args_from_managed_source(Some(root.path()))
        .expect("build Cargo arguments");
    let values = args
        .iter()
        .map(|argument| argument.to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    assert!(
        values
            .iter()
            .any(|value| value == "source.crates-io.replace-with=\"mgc-managed\"")
    );
    assert!(values.contains(&format!(
        "source.mgc-managed.directory={}",
        toml::Value::String(root.path().to_string_lossy().into_owned())
    )));
    assert!(values.ends_with(&[
        "run".to_string(),
        "--locked".to_string(),
        "--offline".to_string()
    ]));
}

#[test]
fn native_rust_dev_without_mgc_lock_fails_before_cargo_launch() {
    let root = tempfile::tempdir().expect("create unmanaged Rust project");
    std::fs::create_dir(root.path().join("src")).expect("create source directory");
    std::fs::write(
        root.path().join("Cargo.toml"),
        "[package]\nname = \"unmanaged-web\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("write Cargo manifest");
    std::fs::write(root.path().join("src/main.rs"), "fn main() {}\n")
        .expect("write Rust entrypoint");

    let error = super::infer_native_dev_launch(root.path(), None, None)
        .expect_err("Rust dev must require MGC lock and verified source state");

    assert!(!error.to_string().is_empty());
}

#[test]
fn python_interpreter_name_detection_covers_supported_executable_names() {
    for name in ["python", "python3", "python3.15", "python315", "python.exe"] {
        assert!(
            super::super::dev::is_python_interpreter_name(name),
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
            !super::super::dev::is_python_interpreter_name(name),
            "expected {name} to stay outside Python interpreter detection"
        );
    }
}
