//! Exercises deterministic PATH lookup without launching discovered programs.
//! Kiểm tra tìm PATH xác định mà không chạy chương trình tìm thấy.

use super::super::{
    MAX_VERSION_LINE_BYTES, TOOLCHAIN_PROBES, VERSION_PROBE_ENV_KEYS, VERSION_TRUNCATION_MARKER,
    VERSION_UNAVAILABLE, inventory, inventory_for_core, pin_candidate, version_from_stdout,
    version_probe_environment, version_probe_options, which_in_path, which_in_paths,
};

#[test]
fn inventory_keeps_the_closed_tool_order_and_fixed_probe_arguments() {
    assert_eq!(
        TOOLCHAIN_PROBES,
        [
            ("node", "node", &["--version"][..]),
            ("python3", "python3", &["-S", "--version"][..]),
            ("rustc", "rustc", &["-vV"][..]),
            ("cargo", "cargo", &["--version"][..]),
            ("git", "git", &["--version"][..]),
            ("docker", "docker", &["--version"][..]),
        ]
    );
}

#[test]
fn version_probe_options_are_bounded_and_clear_ambient_runtime_hooks() {
    let options = version_probe_options();

    assert_eq!(options.timeout, Some(std::time::Duration::from_secs(5)));
    assert!(matches!(
        options.execution_scope,
        Some(mgc_exec::allowlist::ExecutionScope::BuildRunner)
    ));
    assert!(options.clean_env);
    assert!(!options.capture_full_stdout);
    assert_eq!(
        VERSION_PROBE_ENV_KEYS,
        &[
            "HOME",
            "CARGO_HOME",
            "RUSTUP_HOME",
            "RUSTUP_TOOLCHAIN",
            "ASDF_DATA_DIR",
            "MISE_DATA_DIR",
            "PYENV_ROOT",
            "VOLTA_HOME",
        ][..]
    );
    assert!(options.env.iter().all(|(key, _)| {
        VERSION_PROBE_ENV_KEYS
            .iter()
            .any(|allowed| key.eq_ignore_ascii_case(allowed))
    }));
    assert!(
        !options
            .env
            .iter()
            .any(|(key, _)| key.eq_ignore_ascii_case("NODE_OPTIONS"))
    );
    assert!(
        !options
            .env
            .iter()
            .any(|(key, _)| key.eq_ignore_ascii_case("PYTHONPATH"))
    );
    assert!(
        !options
            .env
            .iter()
            .any(|(key, _)| key.eq_ignore_ascii_case("PYTHONHOME"))
    );
    assert!(
        !options
            .env
            .iter()
            .any(|(key, _)| key.eq_ignore_ascii_case("PYTHONSTARTUP"))
    );
}

#[test]
fn version_probe_environment_keeps_only_approved_toolchain_selectors() {
    let environment = version_probe_environment();

    assert!(environment.iter().all(|(key, _)| {
        VERSION_PROBE_ENV_KEYS
            .iter()
            .any(|allowed| key.eq_ignore_ascii_case(allowed))
    }));
}

#[test]
fn missing_executable_is_not_reported_as_present() {
    let absent_name = format!("mgc-doctor-absent-{}", uuid::Uuid::new_v4());
    let path = std::env::temp_dir().join(format!("mgc-doctor-path-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&path).unwrap();
    let search_path = path.display().to_string();

    assert_eq!(which_in_path(&absent_name, &search_path), None);

    std::fs::remove_dir_all(path).unwrap();
}

#[test]
fn directory_named_like_executable_is_not_reported_as_present() {
    let path = std::env::temp_dir().join(format!("mgc-doctor-path-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(path.join("node")).unwrap();
    let search_path = path.display().to_string();

    assert_eq!(which_in_path("node", &search_path), None);

    std::fs::remove_dir_all(path).unwrap();
}

#[test]
fn empty_path_does_not_search_the_current_directory() {
    assert_eq!(which_in_path("node", ""), None);
}

#[test]
fn regular_file_candidate_is_found() {
    let path = std::env::temp_dir().join(format!("mgc-doctor-path-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&path).unwrap();
    let executable = path.join("node");
    std::fs::write(&executable, b"not launched by this test").unwrap();
    let search_path = path.display().to_string();

    assert_eq!(
        which_in_path("node", &search_path),
        Some(executable.clone())
    );

    std::fs::remove_dir_all(path).unwrap();
}

#[cfg(target_os = "linux")]
#[test]
fn os_path_search_keeps_non_unicode_directory_components() {
    use std::os::unix::ffi::OsStringExt;

    let root = std::env::temp_dir().join(format!("mgc-doctor-path-{}", uuid::Uuid::new_v4()));
    let path = root.join(std::ffi::OsString::from_vec(vec![b'd', 0xff]));
    std::fs::create_dir_all(&path).unwrap();
    let executable = path.join("node");
    std::fs::write(&executable, b"not launched by this test").unwrap();
    let encoded_path = std::env::join_paths([path.as_path()]).unwrap();

    assert_eq!(
        which_in_paths("node", std::env::split_paths(&encoded_path)),
        Some(executable)
    );

    std::fs::remove_dir_all(root).unwrap();
}

#[cfg(target_os = "macos")]
#[test]
fn os_path_search_keeps_unicode_directory_components() {
    let root = std::env::temp_dir().join(format!("mgc-doctor-path-{}", uuid::Uuid::new_v4()));
    let path = root.join("dír-é");
    std::fs::create_dir_all(&path).unwrap();
    let executable = path.join("node");
    std::fs::write(&executable, b"not launched by this test").unwrap();
    let encoded_path = std::env::join_paths([path.as_path()]).unwrap();

    assert_eq!(
        which_in_paths("node", std::env::split_paths(&encoded_path)),
        Some(executable)
    );

    std::fs::remove_dir_all(root).unwrap();
}

#[cfg(windows)]
#[test]
fn windows_search_checks_the_exe_candidate() {
    let path = std::env::temp_dir().join(format!("mgc-doctor-path-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&path).unwrap();
    let executable = path.join("node.exe");
    std::fs::write(&executable, b"not launched by this test").unwrap();
    let search_path = path.to_string_lossy().into_owned();

    assert_eq!(
        which_in_path("node", &search_path),
        Some(executable.clone())
    );

    std::fs::remove_dir_all(path).unwrap();
}

#[test]
fn selected_candidate_is_pinned_under_an_absolute_parent() {
    let path = std::env::temp_dir().join(format!("mgc-doctor-path-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&path).unwrap();
    let executable = path.join("node");
    std::fs::write(&executable, b"not launched by this test").unwrap();

    let pinned = pin_candidate(&executable).unwrap();

    assert!(pinned.is_absolute());
    assert_eq!(pinned.file_name(), executable.file_name());
    assert!(pinned.is_file());
    std::fs::remove_dir_all(path).unwrap();
}

#[test]
fn path_display_escapes_terminal_control_characters() {
    let untrusted_path = std::path::Path::new("/tmp/tool\u{1b}[31m");

    let displayed = super::super::safe_path_display(untrusted_path);

    assert!(!displayed.contains('\u{1b}'));
}

#[test]
fn version_parser_keeps_one_safe_first_line_and_marks_empty_output() {
    assert_eq!(
        version_from_stdout("  node v22.1.0\nextra details\n"),
        "node v22.1.0"
    );
    assert_eq!(version_from_stdout("\n \t\n"), VERSION_UNAVAILABLE);
    assert!(!version_from_stdout("\u{1b}[2Jnode v22.1.0").contains('\u{1b}'));
}

#[test]
fn version_parser_caps_the_untrusted_display_line() {
    let output = "x".repeat(MAX_VERSION_LINE_BYTES + 20);

    let version = version_from_stdout(&output);

    assert_eq!(version.len(), MAX_VERSION_LINE_BYTES);
    assert!(version.ends_with(VERSION_TRUNCATION_MARKER));
}

#[test]
fn core_inventory_keeps_the_general_tools_and_appends_the_core_tools() {
    let general = inventory();
    let app = inventory_for_core("app");

    // The general six rows are preserved, in order, ahead of the core rows.
    // Sáu hàng chung được giữ nguyên, đúng thứ tự, trước các hàng của core.
    let general_names: Vec<&str> = general.iter().map(|tool| tool.name).collect();
    let app_names: Vec<&str> = app.iter().map(|tool| tool.name).collect();
    assert_eq!(&app_names[..general_names.len()], general_names.as_slice());

    // The app core adds its own tools, each tagged with the core that needs it.
    // Core app thêm tool riêng, mỗi tool gắn core đang cần nó.
    for expected in ["flutter", "gradle", "swift"] {
        let tool = app
            .iter()
            .find(|tool| tool.name == expected)
            .unwrap_or_else(|| panic!("app inventory is missing {expected}"));
        assert_eq!(tool.required_by_core, Some("app"));
    }
}

#[test]
fn core_inventory_tags_a_shared_tool_without_duplicating_it() {
    // `node` is both a general row and a requirement of the web core: it must be
    // tagged in place rather than probed a second time.
    // `node` vừa là hàng chung vừa là yêu cầu của core web: gắn tag tại chỗ,
    // không probe lần hai.
    let web = inventory_for_core("web");
    let node_rows = web.iter().filter(|tool| tool.name == "node").count();

    assert_eq!(node_rows, 1);
    let node = web.iter().find(|tool| tool.name == "node").unwrap();
    assert_eq!(node.required_by_core, Some("web"));
}

#[test]
fn core_inventory_without_a_core_requirement_matches_the_general_inventory() {
    let general = inventory();
    let hardware = inventory_for_core("hardware");

    assert_eq!(hardware.len(), general.len());
    assert!(hardware.iter().all(|tool| tool.required_by_core.is_none()));
}
