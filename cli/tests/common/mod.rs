#![allow(dead_code)]

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

mod local_source;

pub use local_source::local_source;

const MANIFEST: &str = env!("CARGO_MANIFEST_DIR");
const EVIDENCE_HASH_BUFFER_BYTES: usize = 64 * 1024;
const COMMAND_TIMEOUT_SECS: u64 = 300;
const DEV_STARTUP_TIMEOUT_SECS: u64 = 25;
const DEV_COMMAND_TIMEOUT_SECS: u64 = 30;
const WINDOWS_NUXT_DEV_STARTUP_TIMEOUT_SECS: u64 = 90;
const WINDOWS_NUXT_DEV_COMMAND_TIMEOUT_SECS: u64 = 95;
const DEV_PORT_CLOSE_TIMEOUT_SECS: u64 = 5;
const NUXT_WATCH_IGNORE_PROBE: &str = r#"
// Verify the installed Nuxt watcher's filter against MGC storage and application paths.
// Kiểm matcher Nuxt đã cài: bỏ qua storage MGC, vẫn nhận source/build của app.
import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
import { pathToFileURL } from 'node:url';
const require = createRequire(import.meta.url);
const nuxtRequire = createRequire(require.resolve('nuxt/package.json'));
const { loadNuxt, createIsIgnored } = await import(pathToFileURL(nuxtRequire.resolve('@nuxt/kit')).href);
process.env.NUXT_TELEMETRY_DISABLED = '1';
const nuxt = await loadNuxt({ cwd: process.cwd(), dev: true, ready: false });
try {
  await nuxt.runWithContext(() => {
    const ignored = createIsIgnored(nuxt);
    const root = process.cwd().replaceAll('\\', '/');
    for (const path of ['.magicore/cache', '.magicore/cache/web/cas/files/blake3/probe', '.magicore/store', '.magicore/store/probe', '.magicore/exec.log']) {
      assert.equal(ignored(`${root}/${path}`), true, `MGC internal path must be ignored: ${path}`);
    }
    for (const path of ['app.vue', 'server/api/health.ts', '.magicore/build/wasm/application.wasm']) {
      assert.equal(ignored(`${root}/${path}`), false, `application path must remain visible: ${path}`);
    }
  });
} finally {
  await nuxt.close();
}
console.log('MGC_NUXT_CACHE_WATCH_IGNORE_OK');
"#;
static WEB_LIFECYCLE_EVIDENCE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

thread_local! {
    /// Each integration test gets isolated caches shared by its CLI child processes.
    /// Mỗi integration test có cache riêng, dùng chung cho các tiến trình CLI con.
    static ISOLATED_TEST_CACHE: std::cell::OnceCell<tempfile::TempDir> = const {
        std::cell::OnceCell::new()
    };
    /// Keep each test workspace alive until its owning test thread exits.
    /// Giữ workspace của test đến khi thread chạy test kết thúc.
    static ISOLATED_TEST_WORK_DIRS: std::cell::RefCell<Vec<tempfile::TempDir>> = const {
        std::cell::RefCell::new(Vec::new())
    };
}

fn isolated_test_cache_root() -> PathBuf {
    ISOLATED_TEST_CACHE.with(|cache| {
        cache
            .get_or_init(|| tempfile::tempdir().expect("create isolated MGC test cache"))
            .path()
            .to_path_buf()
    })
}

/// Run mgc command from workspace root.
pub fn mgc(args: &[&str]) -> (bool, String) {
    run_mg(args, Path::new(MANIFEST))
}

/// Run mgc command in a specific target directory.
pub fn mgc_in(dir: &Path, args: &[&str]) -> (bool, String) {
    run_mg(args, dir)
}

fn run_mg(args: &[&str], cwd: &Path) -> (bool, String) {
    let bin = mgc_binary();
    let mut options = mgc_exec::run::ExecOptions {
        cwd: Some(cwd.to_path_buf()),
        timeout: Some(std::time::Duration::from_secs(COMMAND_TIMEOUT_SECS)),
        capture_full_stdout: true,
        // A CLI error is evidence returned to the test, never a successful command.
        // Trả mã lỗi cho assertion của test, không coi CLI lỗi là thành công.
        allowed_exit_codes: (1..=255).collect(),
        ..Default::default()
    };
    configure_test_env(&mut options);

    run_binary_with_options(&bin, args, &options)
}

/// Apply the isolated template/cache environment used by every CLI E2E child.
/// Gắn môi trường template/cache cô lập chung cho mọi tiến trình CLI trong E2E.
pub fn configure_test_env(options: &mut mgc_exec::run::ExecOptions) {
    let cache_root = isolated_test_cache_root();
    let isolated_home = cache_root.join("home");
    options
        .env
        .push(("HOME".into(), isolated_home.to_string_lossy().into_owned()));
    options.env.push((
        "USERPROFILE".into(),
        isolated_home.to_string_lossy().into_owned(),
    ));
    options.env.push((
        "APPDATA".into(),
        cache_root.join("app-data").to_string_lossy().into_owned(),
    ));
    options.env.push((
        "LOCALAPPDATA".into(),
        cache_root
            .join("local-app-data")
            .to_string_lossy()
            .into_owned(),
    ));
    options.env.push((
        "MGC_DATA_DIR".into(),
        cache_root
            .join("system-data")
            .to_string_lossy()
            .into_owned(),
    ));
    options.env.push((
        "MAGICORE_STORE_ROOT".into(),
        cache_root.join("store").to_string_lossy().into_owned(),
    ));
    options.env.push((
        "MGC_CACHE_DIR".into(),
        cache_root.join("web-cache").to_string_lossy().into_owned(),
    ));
    options.env.push((
        "MGC_TEMPLATES_DIR".into(),
        cache_root.join("templates").to_string_lossy().into_owned(),
    ));
    options.env.push((
        "MGC_SCAFFOLDS_DIR".into(),
        cache_root.join("scaffolds").to_string_lossy().into_owned(),
    ));
    if let Some(python_version) = std::env::var("MGC_PYTHON_VERSION")
        .ok()
        .or_else(host_python_minor_version)
    {
        options
            .env
            .push(("MGC_PYTHON_VERSION".into(), python_version));
    }
    options.env.push((
        "XDG_CACHE_HOME".into(),
        cache_root.join("xdg-cache").to_string_lossy().into_owned(),
    ));
    options.env.push((
        "XDG_CONFIG_HOME".into(),
        cache_root.join("xdg-config").to_string_lossy().into_owned(),
    ));
    options.env.push((
        "XDG_DATA_HOME".into(),
        cache_root.join("xdg-data").to_string_lossy().into_owned(),
    ));
    // Keep the evidence destination inside the Rust test harness only.
    // Chỉ harness Rust được biết đường dẫn evidence; CLI và project child nhận giá trị rỗng.
    options
        .env
        .push(("MGC_STRESS_LIFECYCLE_EVIDENCE".into(), String::new()));

    let workspace_root = Path::new(MANIFEST)
        .parent()
        .expect("CLI manifest should have a parent");

    // Only pin workspace templates when the tree actually holds template
    // content — the repo may keep just placeholder READMEs and rely on the
    // registry cache (~/.mgc/templates).
    let template_disk = workspace_root.join("templates");
    let template_contract = template_disk
        .join("web")
        .join("frontend")
        .join("react-vite")
        .join("template.toml")
        .is_file();

    if template_contract {
        options.env.push((
            "MAGICORE_TEMPLATE_DIR".into(),
            template_disk.to_string_lossy().into_owned(),
        ));
    }
}

/// Detect the interpreter used by native Python lifecycle tests.
/// Phát hiện Python mà các test vòng đời native sẽ chạy để chọn wheel ABI đúng.
pub fn host_python_minor_version() -> Option<String> {
    static PYTHON_VERSION: std::sync::OnceLock<Option<String>> = std::sync::OnceLock::new();
    PYTHON_VERSION
        .get_or_init(|| {
            let executable = if cfg!(windows) { "python" } else { "python3" };
            let output = std::process::Command::new(executable)
                .args([
                    "-c",
                    "import sys; print(f'{sys.version_info.major}.{sys.version_info.minor}')",
                ])
                .output()
                .ok()?;
            if !output.status.success() {
                return None;
            }
            let version = String::from_utf8(output.stdout).ok()?.trim().to_string();
            let mut parts = version.split('.');
            let major = parts.next()?.parse::<u32>().ok()?;
            let minor = parts.next()?.parse::<u32>().ok()?;
            (major > 0 && parts.next().is_none()).then_some(format!("{major}.{minor}"))
        })
        .clone()
}

/// Resolve the CLI binary Cargo built for integration tests.
/// Tìm binary CLI do Cargo build cho integration tests.
pub fn mgc_binary() -> PathBuf {
    // Cargo overwrites CARGO_BIN_EXE_mgc; use a test-only override for immutable evidence snapshots.
    // Cargo ghi đè biến của nó; override riêng giúp evidence dùng đúng snapshot đã chọn.
    if let Some(value) = std::env::var_os("MGC_TEST_CLI_BINARY") {
        let binary = PathBuf::from(value);
        assert!(
            binary.is_file(),
            "explicit lifecycle CLI snapshot does not exist"
        );
        eprintln!("MagiCore lifecycle product binary: {}", binary.display());
        return binary;
    }
    let runtime_bin = std::env::var("CARGO_BIN_EXE_mgc")
        .ok()
        .map(PathBuf::from)
        .filter(|path| path.exists());
    let compile_bin = option_env!("CARGO_BIN_EXE_mgc")
        .map(PathBuf::from)
        .filter(|path| path.exists());
    runtime_bin
        .or(compile_bin)
        .expect("Cargo must provide CARGO_BIN_EXE_mgc for CLI integration tests")
}

pub fn run_binary_with_options(
    bin: &Path,
    args: &[&str],
    options: &mgc_exec::run::ExecOptions,
) -> (bool, String) {
    // Windows framework E2E must exercise the product guard without nesting the harness guard.
    // Windows framework E2E cần chạy guard của sản phẩm, không lồng guard của harness.
    if cfg!(windows) && std::env::var("MGC_TEST_DIRECT_PRODUCT").is_ok_and(|value| value == "1") {
        return run_product_binary_direct_bounded(bin, args, options);
    }

    // Reuse the platform's bounded process-tree executor instead of unbounded output().
    // Dùng executor có timeout và kill cây tiến trình thay output() không giới hạn.
    let arguments = args
        .iter()
        .map(|arg| (*arg).to_string())
        .collect::<Vec<_>>();
    let output = match mgc_exec::run::run_project_binary(bin, &arguments, options) {
        Ok(report) => report,
        Err(error) => return (false, format!("MGC test command failed: {error:#}")),
    };
    let stdout = output.stdout_full;
    let stderr = output.stderr_tail;
    let combined = if stderr.is_empty() {
        stdout
    } else {
        format!("{stdout}\n{stderr}")
    };
    (output.exit_code == 0, combined)
}

/// Invoke the product binary directly with bounded output and the configured deadline.
/// Chạy thẳng binary sản phẩm, giới hạn capture và tôn trọng deadline cấu hình.
pub fn run_product_binary_direct_bounded(
    bin: &Path,
    args: &[&str],
    options: &mgc_exec::run::ExecOptions,
) -> (bool, String) {
    use std::process::Stdio;

    let mut command = std::process::Command::new(bin);
    command
        .args(args)
        .current_dir(
            options
                .cwd
                .as_deref()
                .unwrap_or_else(|| Path::new(MANIFEST)),
        )
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env_remove("MGC_TEST_DIRECT_PRODUCT");
    if options.clean_env {
        command.env_clear();
        for (key, value) in &options.env {
            if !key.eq_ignore_ascii_case("PATH") {
                command.env(key, value);
            }
        }
        let path = options
            .env
            .iter()
            .rev()
            .find(|(key, _)| key.eq_ignore_ascii_case("PATH"))
            .map(|(_, value)| std::ffi::OsString::from(value))
            .or_else(|| std::env::var_os("PATH"));
        if let Some(path) = path {
            command.env("PATH", path);
        }
        #[cfg(windows)]
        for key in [
            "SYSTEMROOT",
            "WINDIR",
            "TEMP",
            "TMP",
            "USERPROFILE",
            "APPDATA",
            "LOCALAPPDATA",
            "ProgramData",
        ] {
            if let Some(value) = std::env::var_os(key) {
                command.env(key, value);
            }
        }
    } else {
        for (key, value) in &options.env {
            command.env(key, value);
        }
    }

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            return (
                false,
                format!("could not start MGC product binary: {error}"),
            );
        }
    };
    let stdout = child.stdout.take().expect("stdout was configured as piped");
    let stderr = child.stderr.take().expect("stderr was configured as piped");
    let stdout_reader = std::thread::spawn(move || read_direct_process_output_bounded(stdout));
    let stderr_reader = std::thread::spawn(move || read_direct_process_output_bounded(stderr));

    let timeout = (!options.disable_timeout).then(|| {
        options
            .timeout
            .unwrap_or_else(|| Duration::from_secs(COMMAND_TIMEOUT_SECS))
    });
    let deadline = timeout.map(|timeout| Instant::now() + timeout);
    let (status, timed_out) = loop {
        match child.try_wait() {
            Ok(Some(status)) => break (status, false),
            Ok(None) if deadline.is_some_and(|deadline| Instant::now() >= deadline) => {
                #[cfg(windows)]
                terminate_direct_process_tree(&mut child);
                #[cfg(not(windows))]
                {
                    let _ = child.kill();
                    let _ = child.wait();
                }
                break (child.wait().expect("wait for timed-out MGC process"), true);
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            Err(error) => {
                #[cfg(windows)]
                terminate_direct_process_tree(&mut child);
                #[cfg(not(windows))]
                {
                    let _ = child.kill();
                }
                let _ = child.wait();
                return (
                    false,
                    format!("could not poll MGC product process: {error}"),
                );
            }
        }
    };

    let stdout = stdout_reader
        .join()
        .map_err(|_| "stdout reader thread panicked".to_string())
        .and_then(|result| result.map_err(|error| format!("could not read MGC stdout: {error}")));
    let stderr = stderr_reader
        .join()
        .map_err(|_| "stderr reader thread panicked".to_string())
        .and_then(|result| result.map_err(|error| format!("could not read MGC stderr: {error}")));
    let (stdout, stderr) = match (stdout, stderr) {
        (Ok(stdout), Ok(stderr)) => (stdout, stderr),
        (Err(error), _) | (_, Err(error)) => return (false, error),
    };
    let mut output = String::from_utf8_lossy(&stdout).into_owned();
    if !stderr.is_empty() {
        if !output.is_empty() {
            output.push('\n');
        }
        output.push_str(&String::from_utf8_lossy(&stderr));
    }
    if timed_out {
        let timeout_secs = timeout.map_or(0, |timeout| timeout.as_secs());
        output.push_str(&format!(
            "\ncommand timed out after {timeout_secs} seconds while waiting for the MGC product process"
        ));
    }
    (status.success() && !timed_out, output)
}

fn read_direct_process_output_bounded(mut reader: impl Read) -> std::io::Result<Vec<u8>> {
    const MAX_CAPTURE_BYTES: usize = 8 * 1024 * 1024;
    let mut captured = Vec::new();
    let mut buffer = [0u8; 16 * 1024];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(captured);
        }
        let remaining = MAX_CAPTURE_BYTES.saturating_sub(captured.len());
        captured.extend_from_slice(&buffer[..bytes_read.min(remaining)]);
    }
}

#[cfg(windows)]
fn terminate_direct_process_tree(child: &mut std::process::Child) {
    use std::process::Stdio;

    if let Some(system_root) = std::env::var_os("SystemRoot") {
        let taskkill = PathBuf::from(system_root)
            .join("System32")
            .join("taskkill.exe");
        if taskkill.is_file() {
            let pid = child.id().to_string();
            let _ = std::process::Command::new(taskkill)
                .args(["/PID", pid.as_str(), "/T", "/F"])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
    }
    let _ = child.kill();
}

/// Create a temp directory for scaffold testing; remove it when this test thread exits.
/// Tạo thư mục tạm cho scaffold; tự xóa khi thread test này kết thúc.
pub fn work_dir() -> PathBuf {
    let prefix = format!("mgc-test-{}-", std::process::id());
    let workspace = tempfile::Builder::new()
        .prefix(&prefix)
        .tempdir()
        .expect("create work dir");
    let path = workspace.path().to_path_buf();
    ISOLATED_TEST_WORK_DIRS.with(|workspaces| workspaces.borrow_mut().push(workspace));
    path
}

/// Scaffold a project in a temp directory and return its path.
/// Note: --dir flag is not supported; project is always created in CWD.
pub fn scaffold(framework: &str, project: &str) -> PathBuf {
    let base = work_dir();
    // A framework with an official initializer must be scaffolded through it:
    // that is the product path, and it is what these tests exist to exercise.
    // A framework without one falls back to a local project, which is the only
    // other source MagiCore accepts.
    // Framework có initializer chính thức phải được scaffold qua nó: đó là
    // đường sản phẩm và là thứ các test này kiểm. Framework không có thì dùng
    // project local, nguồn duy nhất còn lại MagiCore chấp nhận.
    let result = if has_official_initializer("web", framework) {
        mgc_in(&base, &["create-web", framework, project, "--ts"])
    } else {
        let source = local_source(&base, "web", framework);
        mgc_in(
            &base,
            &["create-web", framework, project, "--ts", &source.flag()],
        )
    };
    assert!(result.0, "scaffold {framework} failed:\n{}", result.1);
    base.join(project)
}

/// Whether the framework has an official initializer, which is the product
/// path these tests must exercise.
/// Framework có initializer chính thức không, tức đường sản phẩm mà test phải
/// kiểm.
/// Whether the official initializer for this framework can actually run here.
/// A host-tool initializer needs the language toolchain, and a CI image without
/// it must skip rather than fail: reporting a missing JDK as a MagiCore failure
/// would mislabel the problem.
/// Initializer chính thức cho framework này có chạy được ở đây không. Initializer
/// host-tool cần toolchain ngôn ngữ; image CI thiếu nó phải SKIP chứ không FAIL,
/// vì báo thiếu toolchain thành lỗi MagiCore là dán nhãn sai vấn đề.
pub fn initializer_available(framework: &str) -> bool {
    match framework {
        "django" => which("django-admin"),
        "laravel" => which("composer"),
        _ => has_official_initializer("web", framework),
    }
}

/// Whether a program resolves on `PATH`.
/// Chương trình có resolve được trên `PATH` không.
fn which(program: &str) -> bool {
    let path = std::env::var_os("PATH").unwrap_or_default();
    std::env::split_paths(&path).any(|dir| dir.join(program).is_file())
}

pub fn has_official_initializer(core: &str, framework: &str) -> bool {
    const OFFICIAL: &[(&str, &str)] = &[
        ("web", "vanilla"),
        ("web", "react-vite"),
        ("web", "vue-vite"),
        ("web", "nextjs"),
        ("web", "sveltekit"),
        ("web", "nuxt"),
        ("web", "angular"),
        ("web", "remix"),
        ("web", "astro"),
        ("web", "vite"),
        ("web", "preact"),
        ("web", "lit"),
        ("web", "vue-cli"),
        ("web", "docusaurus"),
        ("web", "vitepress"),
        ("web", "turbo"),
        ("web", "tauri"),
        ("web", "express"),
        ("web", "nestjs"),
        ("web", "nest"),
        ("web", "spring-boot"),
        ("web", "quarkus"),
        ("web", "django"),
        ("web", "laravel"),
    ];
    OFFICIAL.contains(&(core, framework))
}

/// Assert file exists in project directory.
pub fn assert_file_exists(project: &Path, rel_path: &str) {
    let full = project.join(rel_path);
    assert!(
        full.exists(),
        "expected file '{}' not found in '{}'",
        rel_path,
        project.display()
    );
}

/// Assert file contains expected content.
pub fn assert_file_contains(project: &Path, rel_path: &str, expected: &str) {
    let full = project.join(rel_path);
    assert!(full.exists(), "file '{rel_path}' does not exist");
    let content = std::fs::read_to_string(&full).unwrap_or_default();
    assert!(
        content.contains(expected),
        "file '{rel_path}' does not contain:\n  expected: {expected}\n  actual:\n{content}"
    );
}

/// Return a package.json script without invoking a package manager.
/// Đọc script trong package.json mà không gọi package manager.
pub fn package_json_script(project: &Path, name: &str) -> anyhow::Result<Option<String>> {
    let package_path = project.join("package.json");
    let content = std::fs::read_to_string(&package_path)?;
    let manifest: serde_json::Value = serde_json::from_str(&content)?;
    Ok(manifest
        .get("scripts")
        .and_then(|scripts| scripts.get(name))
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned))
}

/// Exercise the native lifecycle contract for a generated web framework project.
/// Kiểm tra hợp đồng vòng đời native trên project web được scaffold.
pub fn assert_native_web_framework_lifecycle(framework: &str, project: &Path, suite: &str) {
    let (ok, output) = mgc_in(project, &["install"]);
    assert!(ok, "{framework} native install failed:\n{output}");
    assert!(
        project.join("node_modules").is_dir(),
        "{framework}: node_modules missing after native install"
    );
    let lock = std::fs::read_to_string(project.join("mgc.lock")).unwrap_or_else(|error| {
        panic!("{framework}: native install did not write mgc.lock: {error}")
    });
    assert!(
        lock.contains("version"),
        "{framework}: mgc.lock is not a lockfile"
    );

    let (ok, output) = mgc_in(project, &["install", "--frozen"]);
    assert!(ok, "{framework} frozen install failed:\n{output}");
    if framework == "nuxt" {
        let probe = project.join(".magicore").join("nuxt-ignore-probe.mjs");
        std::fs::create_dir_all(probe.parent().expect("Nuxt probe has a parent directory"))
            .expect("create Nuxt watcher probe directory");
        std::fs::write(&probe, NUXT_WATCH_IGNORE_PROBE).expect("write Nuxt watcher probe");
        let (ok, output) = mgc_in(
            project,
            &["exec", "node", "--", ".magicore/nuxt-ignore-probe.mjs"],
        );
        std::fs::remove_file(&probe).expect("remove Nuxt watcher probe");
        assert!(
            ok && output.contains("MGC_NUXT_CACHE_WATCH_IGNORE_OK"),
            "Nuxt watcher filter failed:\n{output}"
        );
    }
    record_web_lifecycle_evidence(
        framework,
        suite,
        "framework-cli-install",
        "native install and frozen replay succeeded",
    );
    assert!(
        package_json_script(project, "test")
            .unwrap_or_else(|error| panic!("{framework}: invalid package.json: {error}"))
            .is_some(),
        "{framework}: scaffold must provide a test script"
    );
    let (ok, output) = mgc_in(project, &["run", "--core", "web", "test"]);
    assert!(ok, "{framework} mgc run test failed:\n{output}");
    record_web_lifecycle_evidence(
        framework,
        suite,
        "framework-cli-run",
        "mgc run executed the scaffold test script",
    );
    let (ok, output) = mgc_in(project, &["test"]);
    assert!(ok, "{framework} mgc test failed:\n{output}");
    record_web_lifecycle_evidence(
        framework,
        suite,
        "framework-cli-test",
        "mgc test completed the scaffold test script",
    );

    let build_script = package_json_script(project, "build")
        .unwrap_or_else(|error| panic!("{framework}: invalid package.json: {error}"));
    if build_script.is_some() {
        let (ok, output) = mgc_in(project, &["build"]);
        assert!(ok, "{framework} mgc build failed:\n{output}");
        record_web_lifecycle_evidence(
            framework,
            suite,
            "framework-cli-build",
            "mgc build completed the scaffold build script",
        );
    } else {
        record_web_lifecycle_evidence(
            framework,
            suite,
            "framework-cli-build-not-defined",
            "scaffold package.json does not declare a build script",
        );
    }

    assert_native_web_dev_server_lifecycle(framework, project, suite);
}

fn record_web_lifecycle_evidence(framework: &str, suite: &str, category: &str, detail: &str) {
    let Some(path) = std::env::var_os("MGC_STRESS_LIFECYCLE_EVIDENCE") else {
        return;
    };
    let binary = mgc_binary()
        .canonicalize()
        .expect("canonicalize lifecycle-test CLI binary");
    // Bind records to executable bytes, not merely a reusable filesystem path.
    // Gắn record với byte executable để binary thay tại cùng path không dùng được evidence cũ.
    use sha2::Digest;
    let mut executable = std::fs::File::open(&binary).expect("open lifecycle CLI binary");
    let mut digest = sha2::Sha256::new();
    let mut buffer = [0_u8; EVIDENCE_HASH_BUFFER_BYTES];
    loop {
        let length = executable
            .read(&mut buffer)
            .expect("hash lifecycle CLI binary");
        if length == 0 {
            break;
        }
        digest.update(&buffer[..length]);
    }
    let binary_sha256 = hex::encode(digest.finalize());
    let record = serde_json::json!({
        "schema_version": 1,
        "core": "web",
        "framework": framework,
        "suite": suite,
        "binary_path": binary,
        "binary_sha256": binary_sha256,
        "category": category,
        "passed": true,
        "returncode": 0,
        "detail": detail,
    });
    let line = serde_json::to_string(&record).expect("serialize lifecycle evidence");
    let _guard = WEB_LIFECYCLE_EVIDENCE_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("open lifecycle evidence output");
    writeln!(file, "{line}").expect("write lifecycle evidence record");
}

/// Assign a unique, RULE §13-approved port to each supported native web framework.
/// Cấp cổng duy nhất, thuộc RULE §13, cho từng framework web native được hỗ trợ.
pub fn native_web_dev_port(framework: &str) -> Option<u16> {
    Some(match framework {
        "vanilla" => 5143,
        "ts" => 4351,
        "node" => 4153,
        "angular" => 4531,
        "astro" => 4135,
        "express" => 4513,
        "fastify" => 3415,
        "fastapi" => 5341,
        "hono" => 1354,
        "nestjs" => 1435,
        "nextjs" => 3451,
        "nuxt" => 1453,
        "qwik" => 1534,
        "react-vite" => 3145,
        "remix" => 3154,
        "solidjs" => 1543,
        "sveltekit" => 3541,
        "trpc" => 5134,
        "vue-vite" => 3514,
        _ => return None,
    })
}

/// Select bounded startup and shutdown budgets for native dev smoke tests.
/// Chọn budget startup và dừng process cho smoke test dev native.
pub fn native_web_dev_timeouts(framework: &str, is_windows: bool) -> (u64, u64) {
    // Cold Nuxt compilation on Windows needs time beyond the first loading response.
    // Nuxt Windows cold-start cần vượt trang loading; chỉ HTTP thành công mới là pass.
    if is_windows && framework == "nuxt" {
        (
            WINDOWS_NUXT_DEV_STARTUP_TIMEOUT_SECS,
            WINDOWS_NUXT_DEV_COMMAND_TIMEOUT_SECS,
        )
    } else {
        (DEV_STARTUP_TIMEOUT_SECS, DEV_COMMAND_TIMEOUT_SECS)
    }
}

/// Require HTTP readiness and complete process-tree shutdown within the lane budget.
/// Bắt HTTP ready và cây process dừng sạch trong budget của lane.
pub fn assert_native_web_dev_server_lifecycle(framework: &str, project: &Path, suite: &str) {
    let (startup_timeout_secs, command_timeout_secs) =
        native_web_dev_timeouts(framework, cfg!(windows));
    let port = native_web_dev_port(framework)
        .unwrap_or_else(|| panic!("missing native dev smoke port for {framework}"));
    // Serialize cooperating test processes for this fixed RULE-approved port.
    // Khóa theo port giữa test process; ứng dụng ngoài test vẫn bị phát hiện.
    let port_lock = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(std::env::temp_dir().join(format!("mgc-e2e-dev-port-{port}.lock")))
        .expect("open native dev port test lock");
    const PORT_LOCK_WAIT_SECS: u64 = 120;
    let lock_deadline = std::time::Instant::now() + Duration::from_secs(PORT_LOCK_WAIT_SECS);
    loop {
        match fs2::FileExt::try_lock_exclusive(&port_lock) {
            Ok(()) => break,
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                assert!(
                    std::time::Instant::now() < lock_deadline,
                    "timed out waiting for native dev test port {port}"
                );
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(error) => panic!("cannot lock native dev test port {port}: {error}"),
        }
    }
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    assert!(
        TcpStream::connect_timeout(&address, Duration::from_millis(100)).is_err(),
        "{framework} dev smoke port {port} is already occupied"
    );

    let binary = mgc_binary();
    let mut options = mgc_exec::run::ExecOptions {
        cwd: Some(project.to_path_buf()),
        timeout: Some(Duration::from_secs(command_timeout_secs)),
        capture_full_stdout: true,
        allowed_exit_codes: (1..=255).collect(),
        ..Default::default()
    };
    configure_test_env(&mut options);
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || {
        let port_arg = port.to_string();
        let args = ["dev", "--host", "127.0.0.1", "--port", port_arg.as_str()];
        let result = run_binary_with_options(&binary, &args, &options);
        let _ = sender.send(result);
    });

    let startup_deadline = Instant::now() + Duration::from_secs(startup_timeout_secs);
    let mut response_status = None;
    let mut last_http_status = None;
    while Instant::now() < startup_deadline {
        match receiver.try_recv() {
            Ok((ok, output)) => {
                panic!("{framework} mgc dev exited before serving a request (ok={ok}):\n{output}")
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                panic!("{framework} mgc dev worker exited before serving a request")
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
        }
        if let Some(status) = dev_http_status(address) {
            last_http_status = Some(status);
            if (200..400).contains(&status) {
                response_status = Some(status);
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    let status = match response_status {
        Some(status) => status,
        None => {
            let cleanup_wait = command_timeout_secs
                .saturating_sub(startup_timeout_secs)
                .saturating_add(DEV_PORT_CLOSE_TIMEOUT_SECS);
            let (ok, output) = receiver
                .recv_timeout(Duration::from_secs(cleanup_wait))
                .unwrap_or_else(|error| {
                    panic!(
                        "{framework} mgc dev did not finish cleanup after startup timeout: {error}"
                    )
                });
            panic!(
                "{framework} mgc dev did not serve a successful HTTP response before the startup deadline (last status: {last_http_status:?}, ok={ok}):\n{output}"
            );
        }
    };

    let (ok, output) = receiver
        .recv_timeout(Duration::from_secs(
            command_timeout_secs + DEV_PORT_CLOSE_TIMEOUT_SECS,
        ))
        .unwrap_or_else(|error| panic!("{framework} mgc dev did not stop after timeout: {error}"));
    assert!(
        !ok && output.contains("command timed out after"),
        "{framework} mgc dev should stop through the bounded process-tree timeout:\n{output}"
    );
    assert!(
        (200..400).contains(&status),
        "{framework} dev returned unexpected HTTP status {status}"
    );

    let close_deadline = Instant::now() + Duration::from_secs(DEV_PORT_CLOSE_TIMEOUT_SECS);
    while Instant::now() < close_deadline {
        if TcpStream::connect_timeout(&address, Duration::from_millis(100)).is_err() {
            record_web_lifecycle_evidence(
                framework,
                suite,
                "framework-cli-dev",
                "mgc dev served HTTP and its bounded process tree exited",
            );
            return;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    panic!("{framework} dev listener remained open after its process tree was stopped");
}

fn dev_http_status(address: SocketAddr) -> Option<u16> {
    let mut stream = TcpStream::connect_timeout(&address, Duration::from_millis(200)).ok()?;
    stream.set_read_timeout(Some(Duration::from_secs(1))).ok()?;
    stream
        .write_all(b"GET / HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
        .ok()?;
    let mut response = [0_u8; 256];
    let read = stream.read(&mut response).ok()?;
    let first_line = std::str::from_utf8(&response[..read])
        .ok()?
        .lines()
        .next()?;
    first_line.split_whitespace().nth(1)?.parse().ok()
}

/// Time scaffold execution, return duration in ms.
pub fn bench_scaffold(framework: &str, project: &str) -> u128 {
    let base = work_dir();
    // Benchmark the scaffold pipeline itself, not registry latency: the source
    // is an existing project of the same framework.
    // Đo chính pipeline scaffold, không phải độ trễ registry: nguồn là project
    // có sẵn cùng framework.
    let start = Instant::now();
    let result = if has_official_initializer("web", framework) {
        mgc_in(&base, &["create-web", framework, project, "--ts"])
    } else {
        let source = local_source(&base, "web", framework);
        mgc_in(
            &base,
            &["create-web", framework, project, "--ts", &source.flag()],
        )
    };
    let elapsed = start.elapsed().as_millis();
    assert!(result.0, "bench scaffold {framework} failed: {}", result.1);
    elapsed
}

pub fn assert_help_contains(expected: &str) {
    let (ok, out) = mgc(&["--help"]);
    assert!(ok, "mgc --help failed");
    assert!(
        out.contains(expected),
        "mgc --help should contain '{expected}'\n---\n{out}"
    );
}

pub fn assert_help_excludes(unexpected: &str) {
    let (ok, out) = mgc(&["--help"]);
    assert!(ok, "mgc --help failed");
    assert!(
        !out.contains(unexpected),
        "mgc --help should NOT contain '{unexpected}'\n---\n{out}"
    );
}

/// Default dev port
pub const DEV_PORT: &str = "4315";

/// Publish times for an e2e registry fixture that models a settled package.
///
/// The default 24h release-age quarantine is fail-closed, so a mocked
/// packument must say when each version appeared — exactly like a real
/// registry does. Without this map the resolver cannot prove a release left
/// quarantine and refuses the fixture, which would make every install e2e
/// fail for a reason unrelated to what it tests.
/// (Timestamp publish cho fixture registry e2e mô phỏng package đã ổn định.
/// Quarantine mặc định 24h fail-closed nên packument mock phải có map `time`
/// như registry thật; thiếu nó resolver không chứng minh được release đã qua
/// quarantine và từ chối fixture.)
///
/// `versions` must name every version the fixture serves: one old timestamp
/// is written per entry.
/// (`versions` phải nêu đủ mọi version fixture phục vụ.)
#[allow(dead_code)] // Each e2e binary includes this module and uses only what it needs.
pub fn seeded_publish_times(versions: &[&str]) -> serde_json::Value {
    let times: serde_json::Map<String, serde_json::Value> = versions
        .iter()
        .map(|version| {
            (
                (*version).to_string(),
                serde_json::Value::String("2020-01-01T00:00:00.000Z".to_string()),
            )
        })
        .collect();
    serde_json::Value::Object(times)
}
