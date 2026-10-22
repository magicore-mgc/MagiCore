//! Security tests for ExecutionScope enforcement in test runners.
//!
//! Verifies that:
//! - Project test scripts execute through the native task runner
//!   (Script test của project chạy qua task runner native.)
//! - Package managers remain forbidden in native test and install scopes
//!   (Package manager tiếp tục bị chặn trong các scope test và install native.)
//! - cwd lock prevents directory traversal
//! - Shell injection is prevented
//! - Audit log records all tool executions

#![allow(clippy::unwrap_used)]

use std::env;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper: Create a package.json project with a portable Python test script.
/// Tạo project package.json với test script Python chạy trên nhiều OS.
fn create_package_json_project(dir: &TempDir) -> PathBuf {
    let project_root = dir.path().to_path_buf();
    let package_json = project_root.join("package.json");
    let python = if cfg!(windows) { "python" } else { "python3" };
    let script = format!("{python} -c \"print('Test passed')\"");
    let manifest = serde_json::json!({
        "name": "test-project",
        "version": "1.0.0",
        "scripts": { "test": script }
    });
    fs::write(
        &package_json,
        serde_json::to_vec_pretty(&manifest).expect("serialize package.json"),
    )
    .expect("Failed to write package.json");
    project_root
}

/// Helper: Run mgc binary with arguments
fn run_mgc(args: &[&str], cwd: Option<&PathBuf>) -> std::process::Output {
    // CARGO_BIN_EXE_mgc: the just-built binary under test — never a stale
    // target/debug/mgc and never `cargo run` (release evidence must come
    // from the artifact under test, not the source tree).
    // (Binary vừa build đang test — không bao giờ target/debug cũ hay
    // `cargo run`.)
    let mgc_bin = if let Some(value) = env::var_os("MGC_TEST_CLI_BINARY") {
        let path = PathBuf::from(value);
        assert!(
            path.is_file(),
            "MGC_TEST_CLI_BINARY does not exist: {}",
            path.display()
        );
        eprintln!("MagiCore test-runner product binary: {}", path.display());
        path
    } else {
        PathBuf::from(
            env::var("CARGO_BIN_EXE_mgc")
                .expect("CARGO_BIN_EXE_mgc not set — run via `cargo test -p mgc`"),
        )
    };

    let mut cmd = std::process::Command::new(&mgc_bin);
    cmd.args(args);
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    cmd.output().expect("Failed to execute mgc")
}

#[test]
fn test_package_json_script_executes_through_native_runner() {
    // TEST: mgc test executes the project's script through the native runner.
    // TEST: mgc test chạy script của project qua runner native.
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_root = create_package_json_project(&temp_dir);

    // Run the actual product command; npm is neither required nor invoked.
    // Chạy lệnh sản phẩm; npm không phải điều kiện và không được gọi.
    let output = run_mgc(&["test"], Some(&project_root));

    assert!(
        output.status.success(),
        "mgc test must succeed for the declared native script: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("forbidden package manager"),
        "native project script must not route through a package manager, got: {}",
        stderr
    );

    // ASSERT: the project-owned script's output came from the native runner.
    // Xác nhận output của script project do runner native tạo ra.
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Test passed"),
        "Should run the project test script natively, got: {}",
        stdout
    );
    let audit_log = project_root.join(".magicore").join("exec.log");
    let programs = fs::read_to_string(&audit_log)
        .expect("native project script should create an execution audit log")
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).expect("valid audit JSON"))
        .filter_map(|entry| {
            entry
                .get("cmd")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
        })
        .collect::<Vec<_>>();
    assert!(
        programs
            .iter()
            .any(|program| program == "python" || program == "python3"),
        "audit log must show the native Python runner: {programs:?}"
    );
    assert!(
        programs
            .iter()
            .all(|program| !matches!(program.as_str(), "npm" | "pnpm" | "yarn" | "bun")),
        "native test script must not spawn a package manager: {programs:?}"
    );
}

#[cfg(unix)]
#[test]
fn package_script_runs_owned_local_tool_but_still_denies_package_managers() {
    use std::os::unix::fs::PermissionsExt;
    let project = TempDir::new().expect("temporary project");
    let root = project.path().to_path_buf();
    let bin = root.join("node_modules/.bin");
    fs::create_dir_all(&bin).expect("create local binary directory");
    for name in ["vitest", "npm"] {
        let path = bin.join(name);
        fs::write(&path, "#!/bin/sh\nprintf 'OWNED_TEST_RUNNER_OK\\n'\n")
            .expect("write local test runner");
        fs::set_permissions(path, fs::Permissions::from_mode(0o755)).expect("set executable mode");
    }
    fs::write(
        root.join("package.json"),
        r#"{"name":"runner-project","version":"1.0.0","scripts":{"test":"vitest run"}}"#,
    )
    .expect("write test script");

    let output = run_mgc(&["run", "test"], Some(&root));
    assert!(
        output.status.success(),
        "local runner failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("OWNED_TEST_RUNNER_OK"));

    fs::write(
        root.join("package.json"),
        r#"{"name":"runner-project","version":"1.0.0","scripts":{"test":"npm install"}}"#,
    )
    .expect("write forbidden package-manager script");
    let output = run_mgc(&["run", "test"], Some(&root));
    assert!(!output.status.success());
    assert!(!String::from_utf8_lossy(&output.stdout).contains("OWNED_TEST_RUNNER_OK"));
}

#[cfg(unix)]
#[test]
fn package_script_rejects_local_tool_linked_outside_project() {
    use std::os::unix::{fs::PermissionsExt, fs::symlink};
    let project = TempDir::new().expect("temporary project");
    let outside = TempDir::new().expect("external fixture");
    let root = project.path().to_path_buf();
    let external_tool = outside.path().join("vitest");
    fs::write(
        &external_tool,
        "#!/bin/sh\nprintf 'EXTERNAL_RUNNER_EXECUTED\\n'\n",
    )
    .expect("write external runner");
    fs::set_permissions(&external_tool, fs::Permissions::from_mode(0o755))
        .expect("set external executable mode");
    fs::create_dir_all(root.join("node_modules/.bin")).expect("create local bin directory");
    symlink(external_tool, root.join("node_modules/.bin/vitest")).expect("link external runner");
    fs::write(
        root.join("package.json"),
        r#"{"name":"runner-project","version":"1.0.0","scripts":{"test":"vitest run"}}"#,
    )
    .expect("write project manifest");

    let output = run_mgc(&["run", "test"], Some(&root));
    assert!(!output.status.success());
    assert!(!String::from_utf8_lossy(&output.stdout).contains("EXTERNAL_RUNNER_EXECUTED"));
}

#[test]
fn test_npm_forbidden_in_install_scope() {
    // TEST: Package manager (npm) should be FORBIDDEN in Install scope
    // Simulated via direct allowlist check (Install scope used by scaffold)

    use mgc_exec::allowlist::{ExecutionScope, check_tool_with_scope};

    let result = check_tool_with_scope("npm", ExecutionScope::Install, None);

    // ASSERT: Must be rejected
    assert!(result.is_err(), "npm must be forbidden in Install scope");

    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("forbidden") || err_msg.contains("not allowed"),
        "Error message should indicate npm is forbidden, got: {}",
        err_msg
    );
}

#[test]
fn test_package_manager_script_fails_closed_in_native_test_runner() {
    // TEST: a package.json script cannot delegate native testing to pnpm.
    // TEST: script package.json không được chuyển test native sang pnpm.
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_root = create_package_json_project(&temp_dir);
    let manifest_path = project_root.join("package.json");
    let mut manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(&manifest_path).expect("read package.json"))
            .expect("parse package.json");
    manifest["scripts"]["test"] = serde_json::Value::String("pnpm --version".into());
    fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )
    .expect("write package manager script fixture");

    let output = run_mgc(&["test"], Some(&project_root));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "mgc test must reject a package-manager script, stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(
        stderr.contains("forbidden package manager 'pnpm'"),
        "mgc test must identify the rejected package manager, got: {}",
        stderr
    );
}

#[test]
fn test_cwd_lock_sets_working_directory() {
    // TEST: mgc sets correct working directory for child process
    // Project scripts run with user permissions; this proves their configured cwd.
    // Script project chạy bằng quyền user; test này xác nhận cwd được cấu hình.

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let parent_dir = temp_dir.path();

    // Create project root INSIDE temp dir
    let project_root = parent_dir.join("project");
    fs::create_dir(&project_root).expect("Failed to create project dir");

    // Use Python to print the child working directory on all supported OSes.
    // Dùng Python in thư mục làm việc của tiến trình con trên mọi OS hỗ trợ.
    let python = if cfg!(windows) { "python" } else { "python3" };
    let script = format!("{python} -c \"import os; print(os.getcwd())\"");
    let manifest = serde_json::json!({
        "name": "cwd-test",
        "scripts": { "test": script }
    });
    fs::write(
        project_root.join("package.json"),
        serde_json::to_vec_pretty(&manifest).expect("serialize package.json"),
    )
    .expect("Failed to write package.json");

    let output = run_mgc(&["test"], Some(&project_root));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("=== mgc test output ===\n{}{}", stdout, stderr);

    assert!(output.status.success(), "mgc test failed: {stderr}");
    let expected_cwd = project_root.canonicalize().expect("Failed to canonicalize");
    assert!(
        stdout.contains(&expected_cwd.to_string_lossy().to_string()),
        "mgc did not set the project cwd. Expected '{}', got: {}",
        expected_cwd.display(),
        stdout
    );
}

#[test]
fn test_shell_injection_prevented() {
    // TEST: Shell metacharacters in tool names should be rejected
    // NEGATIVE TEST: Try to execute tool with shell injection → must FAIL

    use mgc_exec::allowlist::ExecutionScope;
    use mgc_exec::allowlist::check_tool_with_scope;

    // ATTACK 1: Try to inject shell command via tool name
    let malicious_tools = vec![
        "npm; rm -rf /",          // Command chaining
        "npm && cat /etc/passwd", // Command chaining
        "npm | tee evil.txt",     // Pipe injection
        "npm $(whoami)",          // Command substitution
        "npm `whoami`",           // Command substitution (backticks)
        "npm > /dev/null",        // Redirection
    ];

    for tool in malicious_tools {
        let result = check_tool_with_scope(tool, ExecutionScope::TestRunner, None);

        // ASSERT: Must be rejected (either not in allowlist or contains forbidden chars)
        assert!(
            result.is_err(),
            "SECURITY VULNERABILITY: Shell injection not blocked for tool: '{}'.\n\
            Malicious tool name with shell metacharacters was allowed.\n\
            This could lead to command injection attacks.",
            tool
        );

        let err_msg = result.unwrap_err().to_string();
        println!("Blocked shell injection attempt: '{}' → {}", tool, err_msg);
    }

    // ATTACK 2: Verify mgc-exec uses Command::args() not shell
    // This is verified by code inspection: mgc-exec/run.rs uses command.args(args)
    // No shell involvement means shell metacharacters are passed as literal strings, not interpreted

    println!(
        "Shell injection prevented: mgc-exec uses Command::args(), malicious tool names rejected"
    );
}

#[test]
fn test_audit_log_records_execution() {
    // P0.7 FIX: Audit logging already implemented in mgc-exec
    // Test verifies log file creation and content

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_root = create_package_json_project(&temp_dir);

    // Run: mgc test
    let output = run_mgc(&["test"], Some(&project_root));

    // ASSERT: Command should succeed
    assert!(
        output.status.success(),
        "mgc test failed (required for audit log test):\n{:?}",
        output
    );

    // P0.7 FIX: Verify audit log exists and contains execution
    let audit_log = project_root.join(".magicore").join("exec.log");

    assert!(
        audit_log.exists(),
        "Audit log not found at {:?}\n\
        mgc-exec should create .magicore/exec.log when log_path is provided",
        audit_log
    );

    let log_content = fs::read_to_string(&audit_log).expect("Failed to read audit log");

    // Verify log format: JSON lines with required fields
    let mut found_entry = false;
    for line in log_content.lines() {
        if line.trim().is_empty() {
            continue;
        }

        // Parse as JSON
        let entry: serde_json::Value =
            serde_json::from_str(line).expect("Audit log line is not valid JSON");

        // Verify required fields exist
        assert!(entry.get("cmd").is_some(), "Missing 'cmd' field");
        assert!(entry.get("args").is_some(), "Missing 'args' field");
        assert!(
            entry.get("exit_code").is_some(),
            "Missing 'exit_code' field"
        );
        assert!(entry.get("ts").is_some(), "Missing 'ts' (timestamp) field");
        assert!(
            entry.get("duration_ms").is_some(),
            "Missing 'duration_ms' field"
        );

        // Verify args are redacted (no secrets)
        let args = entry["args"].as_array().expect("args not an array");
        for arg in args {
            let arg_str = arg.as_str().expect("arg not a string");
            assert!(
                !arg_str.contains("secret") && !arg_str.contains("password"),
                "Audit log contains non-redacted secret in args"
            );
        }

        found_entry = true;
    }

    assert!(found_entry, "Audit log is empty - no executions recorded");

    println!("Audit log verification passed:");
    println!("   - Log file exists at {:?}", audit_log);
    println!("   - Contains valid JSON entries");
    println!("   - Required fields present: cmd, args, exit_code, ts, duration_ms");
    println!("   - Args properly redacted (no secrets)");
}

#[test]
fn test_path_traversal_in_args_rejected() {
    // TEST: Path traversal validator với ALLOWLISTED tool
    // Chứng minh validator chạy TRƯỚC allowlist check

    use mgc_exec::prelude::{ExecOptions, run};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_root = temp_dir.path();

    // Create sensitive file OUTSIDE project root
    let parent = project_root.parent().unwrap();
    let sensitive_file = parent.join("sensitive.txt");
    fs::write(&sensitive_file, "SECRET_DATA").expect("Failed to write sensitive file");

    // Test 1: Dùng tool ĐƯỢC ALLOWLIST (echo) để prove validator chạy
    let result = run(
        "echo",
        &["../sensitive.txt".to_string()],
        &ExecOptions {
            cwd: Some(project_root.to_path_buf()),
            execution_scope: Some(mgc_exec::allowlist::ExecutionScope::TestRunner),
            ..Default::default()
        },
    );

    // ASSERT: Validator PHẢI reject, KHÔNG phải allowlist
    assert!(
        result.is_err(),
        "VALIDATOR BYPASS: Path traversal không bị chặn bởi validator!\n\
        Tool 'echo' được allowlist nhưng path traversal vẫn phải reject."
    );

    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("Path traversal") || err.contains("outside project root"),
        "Error message không đúng. Expected path traversal error, got: {}",
        err
    );

    println!("Path traversal rejected by validator (not allowlist)");
}

#[test]
fn test_validator_allows_legitimate_dotdot_in_non_paths() {
    // TEST: Validator KHÔNG block args hợp lệ chứa ".." (versions, filenames)

    use mgc_exec::prelude::{ExecOptions, run};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_root = temp_dir.path();

    // Test cases: legitimate args có ".." nhưng KHÔNG phải paths
    let test_cases = vec![
        "1.0..2",         // version range
        "config..backup", // filename with ..
        "--option=a..b",  // flag value
    ];

    for arg in test_cases {
        let result = run(
            "echo",
            &[arg.to_string()],
            &ExecOptions {
                cwd: Some(project_root.to_path_buf()),
                execution_scope: Some(mgc_exec::allowlist::ExecutionScope::TestRunner),
                ..Default::default()
            },
        );

        assert!(
            result.is_ok(),
            "FALSE POSITIVE: Validator blocked legitimate arg '{}'\n\
            Args không phải path không được block chỉ vì chứa '..'",
            arg
        );

        println!("Legitimate arg allowed: '{}'", arg);
    }
}

#[test]
fn test_validator_rejects_actual_path_traversal() {
    // TEST: Validator chặn THẬT SỰ path traversal (file tồn tại)

    use mgc_exec::prelude::{ExecOptions, run};

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_root = temp_dir.path();

    // Create subdir trong project
    let subdir = project_root.join("subdir");
    fs::create_dir(&subdir).unwrap();

    // Create file TRONG project
    let legit_file = project_root.join("legit.txt");
    fs::write(&legit_file, "OK").unwrap();

    // Create file NGOÀI project
    let parent = project_root.parent().unwrap();
    let external_file = parent.join("external.txt");
    fs::write(&external_file, "SECRET").unwrap();

    // Test 1: Path nằm TRONG project → cho phép
    let result = run(
        "echo",
        &["./legit.txt".to_string()],
        &ExecOptions {
            cwd: Some(project_root.to_path_buf()),
            execution_scope: Some(mgc_exec::allowlist::ExecutionScope::TestRunner),
            ..Default::default()
        },
    );
    assert!(result.is_ok(), "Legitimate path trong project bị reject");

    // Test 2: Path escape ra ngoài project → reject
    let result = run(
        "echo",
        &["../external.txt".to_string()],
        &ExecOptions {
            cwd: Some(subdir.clone()),
            execution_scope: Some(mgc_exec::allowlist::ExecutionScope::TestRunner),
            ..Default::default()
        },
    );
    assert!(result.is_err(), "Path traversal escape không bị chặn");

    println!("Validator correctly allows internal paths, blocks escapes");
}
