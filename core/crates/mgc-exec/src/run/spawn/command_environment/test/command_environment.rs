//! Verify guarded environment assignment without launching a child.
//! Kiểm tra gán environment có bảo vệ mà không launch child.

use super::super::configure_command_environment;
use crate::run::implementation::ExecOptions;
use std::ffi::OsStr;
use std::process::Command;

/// Caller variables that must survive into a child command.
/// Biến caller phải sống sót vào child command.
const INHERITED_KEY: &str = "MGC_TEST_INHERITED_VAR";
const INHERITED_VALUE: &str = "caller-value";
const INHERITED_PORT_KEY: &str = "PORT";
const INHERITED_PORT_VALUE: &str = "9999";

#[cfg(test)]
fn assert_explicit_path_is_guarded(clean_env: bool) {
    let root = tempfile::tempdir().expect("create command environment test root");
    let shadow_dir = root.path().join("deny shim directory");
    let caller_bin = root.path().join("caller bin");
    std::fs::create_dir(&shadow_dir).expect("create shadow path");
    std::fs::create_dir(&caller_bin).expect("create caller PATH entry");
    let options = ExecOptions {
        clean_env,
        env: vec![
            ("PATH".to_owned(), caller_bin.to_string_lossy().into_owned()),
            ("MGC_TEST_ENV".to_owned(), "retained".to_owned()),
        ],
        ..ExecOptions::default()
    };
    let mut command = Command::new("placeholder-command");

    configure_command_environment(&mut command, &options, &shadow_dir)
        .expect("configure command environment");

    let applied_path = command
        .get_envs()
        .find(|(key, _)| crate::run::implementation::is_path_env_key(&key.to_string_lossy()))
        .and_then(|(_, value)| value)
        .expect("guarded PATH is explicitly configured");
    let path_entries = std::env::split_paths(applied_path).collect::<Vec<_>>();
    assert_eq!(path_entries.first(), Some(&shadow_dir));
    assert!(path_entries.contains(&caller_bin));

    assert_eq!(
        configured(command.get_envs(), "MGC_TEST_ENV"),
        Some("retained")
    );
}

#[cfg(test)]
fn configured<'a>(
    env: impl Iterator<Item = (&'a OsStr, Option<&'a OsStr>)>,
    key: &str,
) -> Option<&'a str> {
    env.filter(|(name, _)| *name == OsStr::new(key))
        .filter_map(|(_, value)| value)
        .last()
        .and_then(OsStr::to_str)
}

#[cfg(test)]
fn configure(options: &ExecOptions) -> (tempfile::TempDir, Command) {
    let root = tempfile::tempdir().expect("create command environment test root");
    let shadow_dir = root.path().join("deny shim directory");
    std::fs::create_dir(&shadow_dir).expect("create shadow path");
    let mut command = Command::new("placeholder-command");
    configure_command_environment(&mut command, options, &shadow_dir)
        .expect("configure command environment");
    (root, command)
}

#[test]
fn inherited_environment_keeps_deny_shims_before_explicit_path() {
    assert_explicit_path_is_guarded(false);
}

#[test]
fn clean_environment_keeps_deny_shims_and_explicit_values() {
    assert_explicit_path_is_guarded(true);
}

#[test]
fn parent_environment_is_inherited_for_application_runtime_variables() {
    // Regression: `mgc run`/`mgc dev` used to wipe PORT/NODE_ENV/DATABASE_URL.
    // (Hồi quy: `mgc run`/`mgc dev` từng xóa PORT/NODE_ENV/DATABASE_URL.)
    // Serialize the process-global writes through the inheritance assertion below.
    // (Tuần tự hóa ghi biến toàn cục process qua phần kiểm kế thừa bên dưới.)
    let _guard = ENV_WRITE_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    // SAFETY: the test process mutates its own environment under a test lock.
    // AN TOÀN: tiến trình test chỉ đổi environment của chính nó dưới khóa test.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var(INHERITED_KEY, INHERITED_VALUE);
        std::env::set_var(INHERITED_PORT_KEY, INHERITED_PORT_VALUE);
    }
    // The default path (`clean_env: false`) is what `mgc run` and `mgc dev` use.
    // (Đường mặc định (`clean_env: false`) là đường `mgc run` và `mgc dev` dùng.)
    let (_root, command) = configure(&ExecOptions::default());
    assert_eq!(
        configured(command.get_envs(), INHERITED_KEY),
        Some(INHERITED_VALUE),
        "PORT/NODE_ENV-style caller variables must reach the child"
    );
    assert_eq!(
        configured(command.get_envs(), INHERITED_PORT_KEY),
        Some(INHERITED_PORT_VALUE),
        "PORT must be inherited so dev servers bind the caller's port"
    );
}

#[test]
fn explicit_environment_replaces_the_inherited_value() {
    let _guard = ENV_WRITE_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    // SAFETY: the test process mutates its own environment under a test lock.
    // AN TOÀN: tiến trình test chỉ đổi environment của chính nó dưới khóa test.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var(INHERITED_PORT_KEY, INHERITED_PORT_VALUE);
    }
    let (_root, command) = configure(&ExecOptions {
        env: vec![("PORT".to_owned(), "4315".to_owned())],
        ..ExecOptions::default()
    });
    let values = command
        .get_envs()
        .filter(|(key, _)| *key == OsStr::new(INHERITED_PORT_KEY))
        .map(|(_, value)| value.and_then(OsStr::to_str))
        .collect::<Vec<_>>();
    assert_eq!(
        values,
        vec![Some("4315")],
        "an explicit assignment must replace, not duplicate, the inherited value"
    );
}

#[test]
fn clean_environment_still_scrubs_unmanaged_caller_variables() {
    let _guard = ENV_WRITE_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    // SAFETY: the test process mutates its own environment under a test lock.
    // AN TOÀN: tiến trình test chỉ đổi environment của chính nó dưới khóa test.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var(INHERITED_KEY, INHERITED_VALUE);
    }
    let (_root, command) = configure(&ExecOptions {
        clean_env: true,
        env: vec![("MGC_TEST_ENV".to_owned(), "retained".to_owned())],
        ..ExecOptions::default()
    });
    assert_eq!(configured(command.get_envs(), INHERITED_KEY), None);
    assert_eq!(
        configured(command.get_envs(), "MGC_TEST_ENV"),
        Some("retained")
    );
}

#[cfg(unix)]
#[test]
fn spawned_child_observes_the_parent_environment() {
    // End-to-end proof: a real child reads the variable through its own process.
    // (Bằng chứng end-to-end: child thật đọc được biến qua process của chính nó.)
    let _guard = ENV_WRITE_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    // SAFETY: the test process mutates its own environment under a test lock.
    // AN TOÀN: tiến trình test chỉ đổi environment của chính nó dưới khóa test.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var(INHERITED_KEY, INHERITED_VALUE);
    }
    let root = tempfile::tempdir().expect("create command environment test root");
    let shadow_dir = root.path().join("deny shim directory");
    std::fs::create_dir(&shadow_dir).expect("create shadow path");
    let mut command = Command::new("/bin/sh");
    command
        .arg("-c")
        .arg(format!("printf %s \"${{{INHERITED_KEY}}}\""));
    configure_command_environment(&mut command, &ExecOptions::default(), &shadow_dir)
        .expect("configure command environment");

    let output = command.output().expect("spawn environment probe");
    assert!(
        output.status.success(),
        "environment probe failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        INHERITED_VALUE,
        "the spawned child did not inherit the caller environment"
    );
}

#[cfg(unix)]
#[test]
fn inherited_path_stays_usable_when_the_caller_supplies_no_path() {
    // Removing PATH from the child is the failure mode that surfaced as
    // `failed to spawn 'node': No such file or directory` once the parent
    // environment was inherited again.
    // (Xóa PATH khỏi child là lỗi hiện ra dưới dạng
    // `failed to spawn 'node': No such file or directory` khi environment cha
    // được kế thừa trở lại.)
    let _root_and_command = configure(&ExecOptions::default());
    let (_, command) = _root_and_command;
    let path = configured(command.get_envs(), "PATH").expect("PATH must be configured");
    let entries = std::env::split_paths(std::ffi::OsStr::new(path)).collect::<Vec<_>>();
    assert!(
        entries.len() > 1,
        "guarded PATH must contain the deny shim directory plus the caller entries: {path}"
    );

    let mut probe = Command::new("/bin/sh");
    probe.arg("-c").arg("command -v sh");
    probe.envs(
        command.get_envs().filter_map(|(key, value)| {
            value.map(|value| (key.to_os_string(), value.to_os_string()))
        }),
    );
    let output = probe.output().expect("spawn the PATH probe");
    assert!(
        output.status.success(),
        "a child configured with the guarded environment cannot resolve /bin/sh"
    );
}

static ENV_WRITE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
