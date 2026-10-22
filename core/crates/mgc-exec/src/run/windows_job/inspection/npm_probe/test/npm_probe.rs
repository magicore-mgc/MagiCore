// Windows npm probe authentication tests.
// Test xác thực npm probe Windows.

#[cfg(windows)]
#[test]
fn windows_npm_probe_exemption_requires_authenticated_native_shim_and_safe_command_shape() {
    use std::ffi::OsString;

    let root = tempfile::tempdir().expect("create Windows native npm shim fixture");
    let shim = root.path().join("npm.exe");
    std::fs::copy(
        std::env::current_exe().expect("resolve test executable"),
        &shim,
    )
    .expect("copy test executable as native shim fixture");
    let shadow_path = super::ShadowPath {
        dir: root.path().to_path_buf(),
        cleaned: true,
        cleanup_attempted: true,
    };
    let probe = super::WindowsProcessSnapshot {
        pid: 201,
        parent_pid: Some(200),
        creation_time: Some(201),
        image_name: "npm.exe".to_owned(),
        command: vec![shim.as_os_str().to_os_string(), OsString::from("--version")],
    };

    assert!(super::is_shadow_windows_npm_version_probe(
        &probe,
        &shim,
        &shadow_path
    ));
    let hard_link_target_argv0 = super::WindowsProcessSnapshot {
        pid: 204,
        parent_pid: Some(200),
        creation_time: Some(204),
        image_name: "npm.exe".to_owned(),
        command: vec![
            std::env::current_exe()
                .expect("resolve hard-link target argv0 fixture")
                .into_os_string(),
            OsString::from("--version"),
        ],
    };
    assert!(super::is_shadow_windows_npm_version_probe(
        &hard_link_target_argv0,
        &shim,
        &shadow_path
    ));

    let starting_probe = super::WindowsProcessSnapshot {
        pid: 201,
        parent_pid: Some(200),
        creation_time: Some(201),
        image_name: "npm.exe".to_owned(),
        command: vec![shim.as_os_str().to_os_string()],
    };
    assert!(super::is_shadow_windows_npm_version_probe(
        &starting_probe,
        &shim,
        &shadow_path
    ));
    let no_arguments = super::WindowsProcessSnapshot {
        command: Vec::new(),
        ..starting_probe
    };
    assert!(!super::is_shadow_windows_npm_version_probe(
        &no_arguments,
        &shim,
        &shadow_path
    ));

    let install = super::WindowsProcessSnapshot {
        pid: 201,
        parent_pid: Some(200),
        creation_time: Some(201),
        image_name: "npm.exe".to_owned(),
        command: vec![shim.as_os_str().to_os_string(), OsString::from("install")],
    };
    assert!(!super::is_shadow_windows_npm_version_probe(
        &install,
        &shim,
        &shadow_path
    ));

    let actual_image = root.path().join("actual-npm.exe");
    std::fs::write(&actual_image, b"different image path")
        .expect("create spoofed image-path fixture");
    assert!(!super::is_shadow_windows_npm_version_probe(
        &probe,
        &actual_image,
        &shadow_path
    ));
    assert_eq!(
        super::windows_npm_probe_authentication(&probe, None, &shadow_path),
        Err("process image path is unavailable")
    );
    assert_eq!(
        super::windows_npm_probe_authentication(&probe, Some(&actual_image), &shadow_path),
        Err("process image hash does not match the running MGC executable")
    );

    let snapshots = [
        super::WindowsProcessSnapshot {
            pid: 200,
            parent_pid: None,
            creation_time: Some(200),
            image_name: "runner.exe".to_owned(),
            command: vec![OsString::from("runner.exe")],
        },
        probe,
    ];
    let image_paths = std::collections::HashMap::from([(201, shim.clone())]);
    let inspected = super::inspect_windows_job_processes_with_image_paths(
        200,
        &[200, 201],
        &snapshots,
        &[],
        Some(&shadow_path),
        &image_paths,
        None,
    )
    .expect("inspect verified Windows probe");
    assert_eq!(inspected.descendants, [(201, None)]);

    let startup_snapshots = [
        super::WindowsProcessSnapshot {
            pid: 200,
            parent_pid: None,
            creation_time: Some(200),
            image_name: "runner.exe".to_owned(),
            command: vec![OsString::from("runner.exe")],
        },
        super::WindowsProcessSnapshot {
            pid: 201,
            parent_pid: Some(200),
            creation_time: Some(201),
            image_name: "npm.exe".to_owned(),
            command: vec![shim.as_os_str().to_os_string()],
        },
    ];
    let inspected = super::inspect_windows_job_processes_with_image_paths(
        200,
        &[200, 201],
        &startup_snapshots,
        &[],
        Some(&shadow_path),
        &image_paths,
        None,
    )
    .expect("inspect authenticated Windows shim while its arguments initialize");
    assert_eq!(inspected.descendants, [(201, None)]);

    let spoofed_image_paths = std::collections::HashMap::from([(201, actual_image.clone())]);
    let inspected = super::inspect_windows_job_processes_with_image_paths(
        200,
        &[200, 201],
        &snapshots,
        &[],
        Some(&shadow_path),
        &spoofed_image_paths,
        None,
    )
    .expect("inspect process with spoofed argv[0]");
    assert_eq!(inspected.descendants, [(201, Some("npm".to_owned()))]);

    std::fs::write(&shim, b"replaced by a different executable")
        .expect("replace shim identity fixture");
    assert!(!super::is_shadow_windows_npm_version_probe(
        &super::WindowsProcessSnapshot {
            pid: 202,
            parent_pid: Some(200),
            creation_time: Some(202),
            image_name: "npm.exe".to_owned(),
            command: vec![shim.as_os_str().to_os_string(), OsString::from("--version")],
        },
        &shim,
        &shadow_path
    ));
}

#[cfg(windows)]
#[test]
fn windows_bare_npm_probe_authenticates_image_and_rejects_mutations() {
    use std::ffi::OsString;
    let root = tempfile::tempdir().expect("create authenticated alias fixture");
    let executable_target = root.path().join("mgc.exe");
    let image = root.path().join("npm.exe");
    std::fs::copy(
        std::env::current_exe().expect("current executable"),
        &executable_target,
    )
    .expect("copy trusted executable fixture");
    if std::fs::hard_link(&executable_target, &image).is_err() {
        std::fs::copy(&executable_target, &image).expect("copy npm alias fixture");
    }
    let shadow = super::ShadowPath {
        dir: root.path().to_path_buf(),
        cleaned: true,
        cleanup_attempted: true,
    };
    let mut process = super::WindowsProcessSnapshot {
        pid: 101,
        parent_pid: Some(100),
        creation_time: Some(101),
        image_name: "npm.exe".to_owned(),
        command: vec![OsString::from("npm"), OsString::from("--version")],
    };
    assert!(super::is_shadow_windows_npm_version_probe(
        &process,
        &executable_target,
        &shadow
    ));
    assert!(super::is_shadow_windows_npm_version_probe(
        &process, &image, &shadow
    ));
    process.command[1] = OsString::from("install");
    assert!(!super::is_shadow_windows_npm_version_probe(
        &process,
        &executable_target,
        &shadow
    ));
    process.command[1] = OsString::from("--version");
    process.command.pop();
    assert!(super::is_shadow_windows_npm_version_probe(
        &process,
        &executable_target,
        &shadow
    ));
    process.command.clear();
    assert!(!super::is_shadow_windows_npm_version_probe(
        &process,
        &executable_target,
        &shadow
    ));
    std::fs::write(&executable_target, b"spoofed binary").expect("replace image fixture");
    assert!(!super::is_shadow_windows_npm_version_probe(
        &process,
        &executable_target,
        &shadow
    ));
}

#[test]
fn windows_probe_argv0_resolves_only_npm_aliases() {
    use std::ffi::OsStr;
    let root = tempfile::tempdir().expect("probe alias directory");
    let shadow = root.path().join("shadow");
    std::fs::create_dir(&shadow).expect("create private shadow directory");
    for alias in [OsStr::new("npm"), OsStr::new("NPM.EXE")] {
        assert_eq!(
            super::windows_npm_probe_argv0_path(alias, &shadow),
            Some(shadow.join("npm.exe"))
        );
    }
    let absolute_alias = root.path().join("npm.exe");
    assert_eq!(
        super::windows_npm_probe_argv0_path(absolute_alias.as_os_str(), &shadow),
        Some(absolute_alias)
    );
    assert!(super::windows_npm_probe_argv0_path(OsStr::new("other.exe"), &shadow).is_none());
}

#[cfg(windows)]
#[test]
fn windows_nested_npm_probe_requires_authenticated_image_and_version_verb() {
    use std::ffi::OsString;

    let original_executable = std::env::current_exe().expect("resolve test executable");
    let dir = super::unique_temp_dir("mgc-exec-shadow-path").expect("create unique temp dir");
    std::fs::create_dir(&dir).expect("create generated nested shadow directory");
    let shim = dir.join("npm.exe");
    std::fs::copy(&original_executable, &shim).expect("copy executor image as Windows npm shim");
    let shadow_path = super::ShadowPath {
        dir: dir.join("outer-executor-shadow"),
        cleaned: true,
        cleanup_attempted: true,
    };
    let probe = super::WindowsProcessSnapshot {
        pid: 301,
        parent_pid: Some(300),
        creation_time: Some(301),
        image_name: "npm.exe".to_owned(),
        command: vec![shim.as_os_str().to_os_string(), OsString::from("--version")],
    };

    assert!(super::is_shadow_windows_npm_version_probe(
        &probe,
        &shim,
        &shadow_path
    ));

    // Windows may report the original path for a hard-linked image while argv[0]
    // retains the private npm.exe alias used to launch it.
    // (Windows có thể báo path file gốc của hard link dù argv[0] giữ alias npm.exe.)
    let hard_link_dir =
        super::unique_temp_dir("mgc-exec-shadow-path").expect("create unique temp dir");
    std::fs::create_dir(&hard_link_dir).expect("create generated hard-link shadow directory");
    let hard_link_image = hard_link_dir.join("mgc.exe");
    std::fs::copy(&original_executable, &hard_link_image)
        .expect("copy executor image to the temp volume");
    let hard_link_shim = hard_link_dir.join("npm.exe");
    std::fs::hard_link(&hard_link_image, &hard_link_shim)
        .expect("create hard-linked Windows npm shim");
    let hard_link_shadow_path = super::ShadowPath {
        dir: hard_link_dir.join("outer-executor-shadow"),
        cleaned: true,
        cleanup_attempted: true,
    };
    let hard_link_probe = super::WindowsProcessSnapshot {
        pid: 302,
        parent_pid: Some(300),
        creation_time: Some(302),
        image_name: "npm.exe".to_owned(),
        command: vec![
            hard_link_shim.as_os_str().to_os_string(),
            OsString::from("--version"),
        ],
    };
    assert!(super::is_shadow_windows_npm_version_probe(
        &hard_link_probe,
        &hard_link_image,
        &hard_link_shadow_path
    ));
    let hard_link_startup_probe = super::WindowsProcessSnapshot {
        command: vec![hard_link_shim.as_os_str().to_os_string()],
        ..hard_link_probe
    };
    assert!(super::is_shadow_windows_npm_version_probe(
        &hard_link_startup_probe,
        &hard_link_image,
        &hard_link_shadow_path
    ));

    // A forged npm path cannot authenticate executable bytes that differ from MGC.
    // (Tên file npm giả không thể xác thực binary có byte khác MagiCore.)
    let forged_dir =
        super::unique_temp_dir("mgc-exec-shadow-path").expect("create unique temp dir");
    std::fs::create_dir(&forged_dir).expect("create forged generated-looking directory");
    let forged_shim = forged_dir.join("npm.exe");
    std::fs::write(&forged_shim, b"untrusted executable bytes")
        .expect("write untrusted probe image");
    let forged_shadow_path = super::ShadowPath {
        dir: forged_dir.join("outer-executor-shadow"),
        cleaned: true,
        cleanup_attempted: true,
    };
    let forged_probe = super::WindowsProcessSnapshot {
        pid: 303,
        parent_pid: Some(300),
        creation_time: Some(303),
        image_name: "npm.exe".to_owned(),
        command: vec![
            forged_shim.as_os_str().to_os_string(),
            OsString::from("--version"),
        ],
    };
    assert!(!super::is_shadow_windows_npm_version_probe(
        &forged_probe,
        &forged_shim,
        &forged_shadow_path
    ));
    let forged_startup_probe = super::WindowsProcessSnapshot {
        command: vec![forged_shim.as_os_str().to_os_string()],
        ..forged_probe
    };
    assert!(!super::is_shadow_windows_npm_version_probe(
        &forged_startup_probe,
        &forged_shim,
        &forged_shadow_path
    ));

    let install = super::WindowsProcessSnapshot {
        pid: 301,
        parent_pid: Some(300),
        creation_time: Some(301),
        image_name: "npm.exe".to_owned(),
        command: vec![shim.as_os_str().to_os_string(), OsString::from("install")],
    };
    assert!(!super::is_shadow_windows_npm_version_probe(
        &install,
        &shim,
        &shadow_path
    ));

    drop(shadow_path);
    std::fs::remove_dir_all(&dir).expect("remove nested Windows shim fixture");
    drop(hard_link_shadow_path);
    std::fs::remove_dir_all(&hard_link_dir).expect("remove hard-link Windows shim fixture");
    drop(forged_shadow_path);
    std::fs::remove_dir_all(&forged_dir).expect("remove forged Windows shim fixture");
}
