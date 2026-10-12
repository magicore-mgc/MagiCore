//! Project exec scopes across the four primary cores.
//! Kiểm scope exec cho bốn core chính, vẫn cấm package-manager mutation.

#![cfg(unix)]
mod common;

const EXEC_SCOPE_TEST_TIMEOUT_SECS: u64 = 30;

fn verify_project_exec_scope(core: &str, manifest_name: &str, manifest: &str) {
    use std::os::unix::fs::PermissionsExt;
    let workspace = tempfile::tempdir().expect("isolated exec project");
    let root = workspace.path();
    std::fs::write(
        root.join("mgc.toml"),
        format!("name = \"exec-scope\"\nversion = \"0.1.0\"\necosystem = \"{core}\"\n"),
    )
    .expect("write core config");
    if core == "ai" {
        let path = root.join("mgc.toml");
        let mut config = std::fs::read_to_string(&path).expect("read core config");
        config.push_str("\n[ai]\nframework = \"python-agent\"\n");
        std::fs::write(path, config).expect("write explicit AI framework");
    }
    std::fs::write(root.join(manifest_name), manifest).expect("write ecosystem manifest");
    let tools = root.join("tools");
    std::fs::create_dir(&tools).expect("create test tools");
    for (name, body) in [
        ("javac", "#!/bin/sh\nprintf 'MGC_EXEC_SCOPE_OK\n'\n"),
        (
            "npm",
            "#!/bin/sh\nprintf 'UNEXPECTED_PM_EXECUTION' > pm-ran\n",
        ),
    ] {
        let path = tools.join(name);
        std::fs::write(&path, body).expect("write canary executable");
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))
            .expect("make canary executable");
    }
    let mut paths = vec![tools];
    if let Some(value) = std::env::var_os("PATH") {
        paths.extend(std::env::split_paths(&value));
    }
    let path = std::env::join_paths(paths).expect("join isolated executable path");
    let mut options = mgc_exec::run::ExecOptions {
        cwd: Some(root.to_path_buf()),
        timeout: Some(std::time::Duration::from_secs(EXEC_SCOPE_TEST_TIMEOUT_SECS)),
        capture_full_stdout: true,
        allowed_exit_codes: (1..=255).collect(),
        ..Default::default()
    };
    common::configure_test_env(&mut options);
    options.env.push((
        "PATH".into(),
        path.into_string().expect("Unicode test path"),
    ));
    let (ok, output) = common::run_binary_with_options(
        &common::mgc_binary(),
        &["exec", "--core", core, "javac", "--", "--version"],
        &options,
    );
    assert!(
        ok && output.contains("MGC_EXEC_SCOPE_OK"),
        "project language exec must use a project scope: {output}"
    );
    let (ok, output) = common::run_binary_with_options(
        &common::mgc_binary(),
        &[
            "exec",
            "--core",
            core,
            "npm",
            "--",
            "install",
            "unexpected-package",
        ],
        &options,
    );
    assert!(!ok, "package-manager mutation must remain denied: {output}");
    assert!(
        !root.join("pm-ran").exists(),
        "denied PM must never execute"
    );
}

#[cfg(feature = "web")]
#[test]
fn web_exec_uses_project_scope_and_keeps_pm_deny() {
    verify_project_exec_scope(
        "web",
        "package.json",
        r#"{"name":"exec-scope","version":"0.1.0","dependencies":{}}"#,
    );
}
#[cfg(feature = "ai")]
#[test]
fn ai_exec_uses_project_scope_and_keeps_pm_deny() {
    verify_project_exec_scope(
        "ai",
        "pyproject.toml",
        "[project]\nname = \"exec-scope\"\nversion = \"0.1.0\"\ndependencies = []\n",
    );
}
#[cfg(feature = "app")]
#[test]
fn app_exec_uses_project_scope_and_keeps_pm_deny() {
    verify_project_exec_scope(
        "app",
        "pubspec.yaml",
        "name: exec_scope\nversion: 0.1.0\ndependencies: {}\n",
    );
}
#[cfg(feature = "lib")]
#[test]
fn lib_exec_uses_project_scope_and_keeps_pm_deny() {
    verify_project_exec_scope(
        "lib",
        "pom.xml",
        "<project><modelVersion>4.0.0</modelVersion><groupId>mgc.probe</groupId><artifactId>exec-scope</artifactId><version>1.0.0</version></project>",
    );
}
