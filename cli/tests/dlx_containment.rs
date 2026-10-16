//! CLI regression for rejecting untrusted dlx cache paths before installation.
//! Kiểm CLI chặn traversal và cache symlink trước khi cài hoặc chạy code.

mod common;

const DLX_REJECTION_TIMEOUT_SECS: u64 = 30;

fn reject_request(root: &std::path::Path, data: &std::path::Path, request: &str) -> String {
    let mut options = mgc_exec::run::ExecOptions {
        cwd: Some(root.to_path_buf()),
        timeout: Some(std::time::Duration::from_secs(DLX_REJECTION_TIMEOUT_SECS)),
        capture_full_stdout: true,
        allowed_exit_codes: (1..=255).collect(),
        ..Default::default()
    };
    common::configure_test_env(&mut options);
    options
        .env
        .push(("MGC_DATA_DIR".into(), data.to_string_lossy().into_owned()));
    // Pass hostile input to the CLI itself; the executor harness rejects it first.
    // Đưa input xấu trực tiếp tới CLI để không nhầm lớp chặn của harness với dlx.
    let stdout = tempfile::tempfile().expect("capture stdout");
    let stderr = tempfile::tempfile().expect("capture stderr");
    let mut child = std::process::Command::new(common::mgc_binary())
        .args(["dlx", request])
        .current_dir(root)
        .envs(options.env)
        .stdout(stdout.try_clone().expect("clone stdout"))
        .stderr(stderr.try_clone().expect("clone stderr"))
        .spawn()
        .expect("launch actual CLI");
    let deadline =
        std::time::Instant::now() + std::time::Duration::from_secs(DLX_REJECTION_TIMEOUT_SECS);
    let status = loop {
        if let Some(status) = child.try_wait().expect("poll CLI") {
            break status;
        }
        if std::time::Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("dlx rejection exceeded timeout");
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    };
    use std::io::{Read, Seek};
    let mut output = String::new();
    for mut file in [stdout, stderr] {
        file.rewind().expect("rewind CLI output");
        file.read_to_string(&mut output).expect("read CLI output");
    }
    assert!(
        !status.success(),
        "unsafe dlx invocation must fail: {output}"
    );
    output
}

#[test]
fn traversal_is_rejected_before_creating_data_directories() {
    let project = tempfile::tempdir().expect("isolated project");
    let data = project.path().join("data");
    let output = reject_request(project.path(), &data, "pkg@../../../../../../escaped");
    assert!(
        output.contains("Invalid dlx") || output.contains("registry"),
        "{output}"
    );
    assert!(
        !data.exists(),
        "invalid request must not create a data directory"
    );
}

#[cfg(unix)]
#[test]
fn cache_symlinks_are_rejected_before_native_install() {
    for components in [
        vec!["cache"],
        vec!["cache", "web"],
        vec!["cache", "web", "dlx"],
    ] {
        let project = tempfile::tempdir().expect("isolated project");
        let outside = tempfile::tempdir().expect("isolated symlink target");
        let data = project.path().join("data");
        std::fs::create_dir(&data).expect("create data root");
        let mut target = data.clone();
        for component in &components[..components.len() - 1] {
            target.push(component);
            std::fs::create_dir(&target).expect("create real ancestor");
        }
        target.push(components.last().expect("cache component"));
        std::os::unix::fs::symlink(outside.path(), target).expect("redirect cache");
        let output = reject_request(project.path(), &data, "semver@7.7.3");
        assert!(
            output.contains("must be a real directory without symlinks"),
            "{output}"
        );
        assert_eq!(
            std::fs::read_dir(outside.path())
                .expect("inspect target")
                .count(),
            0
        );
    }
}
