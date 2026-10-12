// Verify that CLI E2E children keep global MagiCore writes inside a temporary root.
// Kiểm tra tiến trình CLI trong E2E không ghi dữ liệu toàn cục ra ngoài thư mục tạm.

#![allow(clippy::unwrap_used)]

mod common;

fn configured_path(options: &mgc_exec::run::ExecOptions, key: &str) -> std::path::PathBuf {
    options
        .env
        .iter()
        .rev()
        .find(|(name, _)| name == key)
        .map(|(_, value)| std::path::PathBuf::from(value))
        .unwrap_or_else(|| panic!("{key} must be explicitly redirected for CLI tests"))
}

#[test]
fn cli_test_environment_redirects_global_data_and_store_roots() {
    let mut options = mgc_exec::run::ExecOptions::default();
    common::configure_test_env(&mut options);

    let templates = configured_path(&options, "MGC_TEMPLATES_DIR");
    let temporary_root = templates
        .parent()
        .expect("template directory has an isolated parent");
    for key in [
        "HOME",
        "USERPROFILE",
        "APPDATA",
        "LOCALAPPDATA",
        "MGC_DATA_DIR",
        "MAGICORE_STORE_ROOT",
        "MGC_CACHE_DIR",
        "MGC_TEMPLATES_DIR",
        "XDG_CACHE_HOME",
        "XDG_CONFIG_HOME",
        "XDG_DATA_HOME",
    ] {
        let path = configured_path(&options, key);
        assert!(
            path.starts_with(temporary_root),
            "{key} escaped the isolated CLI test root: {}",
            path.display()
        );
    }
}

#[test]
fn cli_test_environment_pins_python_wheel_abi_to_the_runtime() {
    let mut options = mgc_exec::run::ExecOptions::default();
    common::configure_test_env(&mut options);

    let configured = options
        .env
        .iter()
        .rev()
        .find(|(name, _)| name == "MGC_PYTHON_VERSION")
        .map(|(_, value)| value.as_str());
    let expected = std::env::var("MGC_PYTHON_VERSION")
        .ok()
        .or_else(common::host_python_minor_version);
    assert_eq!(configured, expected.as_deref());
}
