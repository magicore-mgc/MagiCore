//! Native `add-lib` E2E: python/go/rust/java add MUST run through MGC itself
//! (resolve native + edit manifest mgc-side) with ZERO toolchain spawn.
//! Canary fakes on PATH prove no pip/cargo/go process ever runs.
//! Python and Java use local registries; Rust and Go still exercise live registries.
//! (Python và Java dùng registry cục bộ; Rust và Go hiện vẫn kiểm tra registry thật.)
//! (Add-lib native E2E: add chạy trong MGC, không spawn toolchain.)

#![allow(clippy::unwrap_used)]

use base64::Engine;
use sha2::{Digest, Sha256};
use std::io::Write;
use std::process::Command;
use tempfile::TempDir;

const SIX_WHEEL_NAME: &str = "six-1.17.0-py3-none-any.whl";

fn mgc_binary() -> String {
    std::env::var_os("CARGO_BIN_EXE_mgc")
        .map(std::path::PathBuf::from)
        .map(|p| p.to_string_lossy().to_string())
        .expect("CARGO_BIN_EXE_mgc unavailable — run via cargo test")
}

struct NoSpawnSandbox {
    bin_dir: TempDir,
    home_dir: TempDir,
    data_dir: TempDir,
    canary_log: std::path::PathBuf,
    pypi_index_url: Option<String>,
    maven_repo_url: Option<String>,
}

impl NoSpawnSandbox {
    fn multi(tools: &[&str]) -> Self {
        let bin_dir = TempDir::new().unwrap();
        let home_dir = TempDir::new().unwrap();
        let data_dir = TempDir::new().unwrap();
        let log_dir = bin_dir.path().join(".canary");
        std::fs::create_dir_all(&log_dir).unwrap();
        let log = log_dir.join("spawned.log");
        for tool in tools {
            let sh = bin_dir.path().join(tool);
            std::fs::write(
                &sh,
                format!(
                    "#!/bin/sh\nprintf '%s\\n' \"{} $*\" >> {}\nexit 0\n",
                    tool,
                    log.display()
                ),
            )
            .unwrap();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&sh).unwrap().permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&sh, perms).unwrap();
            }
        }
        Self {
            bin_dir,
            home_dir,
            data_dir,
            canary_log: log,
            pypi_index_url: None,
            maven_repo_url: None,
        }
    }

    fn with_pypi(tools: &[&str], pypi_index_url: &str) -> Self {
        let mut sandbox = Self::multi(tools);
        sandbox.pypi_index_url = Some(pypi_index_url.to_string());
        sandbox
    }

    fn with_maven(tools: &[&str], maven_repo_url: &str) -> Self {
        let mut sandbox = Self::multi(tools);
        sandbox.maven_repo_url = Some(maven_repo_url.to_string());
        sandbox
    }

    fn run_mgc(&self, args: &[&str], cwd: &std::path::Path) -> (Option<i32>, String, String) {
        let mut cmd = Command::new(mgc_binary());
        cmd.args(args)
            .current_dir(cwd)
            .env("PATH", self.path_env())
            .env("HOME", self.home_dir.path())
            .env("MGC_DATA_DIR", self.data_dir.path())
            .env_remove("MGC_COMPAT_RUNTIME");
        if let Some(index_url) = &self.pypi_index_url {
            cmd.env("MGC_PYPI_INDEX_URL", index_url);
        }
        if let Some(repo_url) = &self.maven_repo_url {
            cmd.env("MGC_MAVEN_REPO_URL", repo_url);
        }
        // Real toolchains must be unreachable: canary shims shadow them,
        // and anything else must fail loudly, not silently succeed.
        let out = cmd.output().expect("spawn mgc");
        (
            out.status.code(),
            String::from_utf8_lossy(&out.stdout).to_string(),
            String::from_utf8_lossy(&out.stderr).to_string(),
        )
    }

    fn path_env(&self) -> std::ffi::OsString {
        let mut paths = vec![self.bin_dir.path().to_path_buf()];
        if let Some(existing) = std::env::var_os("PATH") {
            paths.extend(std::env::split_paths(&existing));
        }
        std::env::join_paths(paths).unwrap()
    }

    fn marker_text(&self) -> String {
        std::fs::read_to_string(&self.canary_log).unwrap_or_default()
    }
}

// Build a valid small pure-Python wheel to verify hash, fetch, and install locally.
// Tạo wheel Python thuần nhỏ, hợp lệ để xác minh hash, fetch và install cục bộ.
fn six_wheel() -> Vec<u8> {
    let module = b"__version__ = '1.17.0'\n";
    let metadata = b"Metadata-Version: 2.1\nName: six\nVersion: 1.17.0\n\n";
    let wheel_metadata =
        b"Wheel-Version: 1.0\nGenerator: magicore-test\nRoot-Is-Purelib: true\nTag: py3-none-any\n";
    let dist_info = "six-1.17.0.dist-info";
    let metadata_path = format!("{dist_info}/METADATA");
    let wheel_path = format!("{dist_info}/WHEEL");
    let record_path = format!("{dist_info}/RECORD");
    let files = [
        ("six.py", module.as_slice()),
        (metadata_path.as_str(), metadata.as_slice()),
        (wheel_path.as_str(), wheel_metadata.as_slice()),
    ];

    let mut record = String::new();
    for (path, contents) in files {
        record.push_str(&format!(
            "{path},sha256={},{}\n",
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(Sha256::digest(contents)),
            contents.len()
        ));
    }
    record.push_str(&format!("{record_path},,\n"));

    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (path, contents) in files {
        writer.start_file(path, options).unwrap();
        writer.write_all(contents).unwrap();
    }
    writer.start_file(record_path, options).unwrap();
    writer.write_all(record.as_bytes()).unwrap();
    writer.finish().unwrap().into_inner()
}

// Serve metadata and wheel bytes locally so this E2E cannot depend on public PyPI.
// Phục vụ metadata và wheel cục bộ để E2E không phụ thuộc PyPI công khai.
struct LocalPyPi {
    _server: mockito::ServerGuard,
    index_url: String,
    _package_metadata: mockito::Mock,
    _version_metadata: mockito::Mock,
    _wheel: mockito::Mock,
}

impl LocalPyPi {
    fn new() -> Self {
        let mut server = mockito::Server::new();
        let index_url = server.url();
        let wheel = six_wheel();
        let wheel_url = format!("{index_url}/files/{SIX_WHEEL_NAME}");
        let metadata = serde_json::json!({
            "info": { "requires_dist": [], "requires_python": ">=3.11" },
            "releases": {
                "1.17.0": [{
                    "filename": SIX_WHEEL_NAME,
                    "url": wheel_url,
                    "packagetype": "bdist_wheel",
                    "digests": { "sha256": hex::encode(Sha256::digest(&wheel)) },
                    "requires_python": ">=3.11"
                }]
            }
        })
        .to_string();

        let package_metadata = server
            .mock("GET", "/pypi/six/json")
            .expect_at_least(1)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(metadata.clone())
            .create();
        let version_metadata = server
            .mock("GET", "/pypi/six/1.17.0/json")
            .expect_at_least(1)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(metadata)
            .create();
        let wheel = server
            .mock("GET", format!("/files/{SIX_WHEEL_NAME}").as_str())
            .expect_at_least(1)
            .with_status(200)
            .with_header("content-type", "application/octet-stream")
            .with_body(wheel)
            .create();

        Self {
            _server: server,
            index_url,
            _package_metadata: package_metadata,
            _version_metadata: version_metadata,
            _wheel: wheel,
        }
    }
}

fn python_lib_project(dir: &std::path::Path) {
    std::fs::write(
        dir.join("mgc.toml"),
        "name = \"nativelib\"\necosystem = \"lib\"\n[lib]\nlanguage = \"python\"\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("pyproject.toml"),
        "[project]\nname = \"nativelib\"\nversion = \"0.1.0\"\nrequires-python = \">=3.11\"\n",
    )
    .unwrap();
}

fn rust_lib_project(dir: &std::path::Path) {
    std::fs::write(
        dir.join("mgc.toml"),
        "name = \"nativelib\"\necosystem = \"lib\"\n[lib]\nlanguage = \"rust\"\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"nativelib\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
}

fn go_lib_project(dir: &std::path::Path) {
    std::fs::write(
        dir.join("mgc.toml"),
        "name = \"nativelib\"\necosystem = \"lib\"\n[lib]\nlanguage = \"go\"\n",
    )
    .unwrap();
    std::fs::write(dir.join("go.mod"), "module nativelib\n\ngo 1.21\n").unwrap();
}

#[test]
fn python_add_lib_runs_inside_mgc_without_spawning_pip() {
    let project = TempDir::new().unwrap();
    python_lib_project(project.path());
    let pypi = LocalPyPi::new();
    let sandbox =
        NoSpawnSandbox::with_pypi(&["pip", "pip3", "uv", "python", "python3"], &pypi.index_url);

    let (code, stdout, stderr) = sandbox.run_mgc(&["add-lib", "six"], project.path());
    let output = format!("{stdout}{stderr}");
    assert_eq!(
        code,
        Some(0),
        "native python `add-lib six` must succeed:\n{output}"
    );
    let pyproject = std::fs::read_to_string(project.path().join("pyproject.toml")).unwrap();
    assert!(
        pyproject.contains("six"),
        "pyproject.toml must pin six (mgc-side edit):\n{pyproject}"
    );
    assert!(
        sandbox.marker_text().is_empty(),
        "ZERO toolchain spawn allowed, got:\n{}",
        sandbox.marker_text()
    );
    let lock = std::fs::read_to_string(project.path().join("mgc.lock")).unwrap_or_default();
    assert!(
        lock.contains("six") && lock.contains("sha256-"),
        "mgc.lock must record six with registry integrity:\n{lock}"
    );
    // Full native lifecycle: install fetches/verifies/unpacks with zero
    // spawn too.
    let (code, stdout, stderr) = sandbox.run_mgc(&["install-lib"], project.path());
    let output = format!("{stdout}{stderr}");
    assert_eq!(
        code,
        Some(0),
        "native python `install-lib` must succeed:\n{output}"
    );
    assert!(
        sandbox.marker_text().is_empty(),
        "ZERO toolchain spawn during install, got:\n{}",
        sandbox.marker_text()
    );
}

#[test]
fn python_v4_add_lib_frozen_replay_uses_the_v4_lock_reader() {
    let project = TempDir::new().unwrap();
    python_lib_project(project.path());
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"nativelib\"\necosystem = \"lib\"\n[lib]\nlanguage = \"python\"\n[lock]\nschema = \"v4\"\n",
    )
    .unwrap();
    let pypi = LocalPyPi::new();
    let sandbox =
        NoSpawnSandbox::with_pypi(&["pip", "pip3", "uv", "python", "python3"], &pypi.index_url);

    let (code, stdout, stderr) = sandbox.run_mgc(&["add-lib", "six"], project.path());
    assert_eq!(code, Some(0), "V4 add failed:\n{stdout}{stderr}");
    let before = std::fs::read(project.path().join("mgc.lock")).unwrap();
    let text = std::str::from_utf8(&before).unwrap();
    assert_eq!(
        mgc_lockfile::migrate::detect_lockfile_version(text).unwrap(),
        4
    );
    let document = mgc_lockfile::canonical::parse_v4_document(text).unwrap();
    assert!(
        mgc_lockfile::policy::verify_v4_math(&document)
            .unwrap()
            .digest_ok
    );

    let (code, stdout, stderr) = sandbox.run_mgc(&["install-lib", "--frozen"], project.path());
    assert_eq!(
        code,
        Some(0),
        "V4 Python frozen replay must use the schema-aware reader:\n{stdout}{stderr}"
    );
    assert_eq!(
        std::fs::read(project.path().join("mgc.lock")).unwrap(),
        before,
        "V4 frozen replay must preserve lock bytes"
    );
    assert!(sandbox.marker_text().is_empty(), "no Python PM may spawn");
}

#[test]
fn rust_add_lib_runs_inside_mgc_without_spawning_cargo() {
    let project = TempDir::new().unwrap();
    rust_lib_project(project.path());
    std::fs::create_dir_all(project.path().join("src")).unwrap();
    std::fs::write(project.path().join("src/lib.rs"), "pub fn marker() {}\n").unwrap();
    let sandbox = NoSpawnSandbox::multi(&["cargo", "rustc"]);

    let (code, stdout, stderr) = sandbox.run_mgc(&["add-lib", "serde_json"], project.path());
    let output = format!("{stdout}{stderr}");
    assert_eq!(
        code,
        Some(0),
        "native rust `add-lib serde_json` must succeed:\n{output}"
    );
    let cargo = std::fs::read_to_string(project.path().join("Cargo.toml")).unwrap();
    assert!(
        cargo.contains("serde_json"),
        "Cargo.toml must pin serde_json (mgc-side edit):\n{cargo}"
    );
    assert!(
        sandbox.marker_text().is_empty(),
        "ZERO toolchain spawn allowed, got:\n{}",
        sandbox.marker_text()
    );
    let lock = std::fs::read_to_string(project.path().join("mgc.lock")).unwrap_or_default();
    assert!(
        lock.contains("serde_json") && lock.contains("sha256-"),
        "mgc.lock must record serde_json with registry integrity:\n{lock}"
    );
    let (code, stdout, stderr) = sandbox.run_mgc(&["install-lib"], project.path());
    let output = format!("{stdout}{stderr}");
    assert_eq!(
        code,
        Some(0),
        "native rust `install-lib` must succeed:\n{output}"
    );
    assert!(
        sandbox.marker_text().is_empty(),
        "ZERO toolchain spawn during install, got:\n{}",
        sandbox.marker_text()
    );
    // Build through MGC with the real Cargo toolchain, but isolate Cargo's
    // home and target output. The managed-source route must stay offline.
    // (Build qua MGC bằng Cargo thật, cô lập Cargo home và target; source
    // managed phải giữ offline.)
    let cargo_home = sandbox.home_dir.path().join("cargo-home");
    let cargo_target = project.path().join(".mgc-cargo-target");
    std::fs::create_dir_all(&cargo_home).unwrap();
    let rustup_home = std::env::var_os("RUSTUP_HOME").or_else(|| {
        std::env::var_os("HOME").map(|home| {
            std::path::PathBuf::from(home)
                .join(".rustup")
                .into_os_string()
        })
    });
    let mut build = Command::new(mgc_binary());
    build
        .arg("build")
        .current_dir(project.path())
        .env("HOME", sandbox.home_dir.path())
        .env("USERPROFILE", sandbox.home_dir.path())
        .env("MGC_DATA_DIR", sandbox.data_dir.path())
        .env("MAGICORE_STORE_ROOT", sandbox.data_dir.path().join("store"))
        .env("CARGO_HOME", &cargo_home)
        .env("CARGO_TARGET_DIR", &cargo_target);
    if let Some(rustup_home) = rustup_home {
        build.env("RUSTUP_HOME", rustup_home);
    }
    let output = build.output().expect("mgc build failed to execute");
    let build_output = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let projected_lock = std::fs::read_to_string(project.path().join("Cargo.lock"))
        .unwrap_or_else(|error| format!("<unreadable Cargo.lock: {error}>"));
    assert!(
        output.status.success(),
        "native Rust build from MGC-managed source failed:\n{build_output}\nMGC Cargo.lock projection:\n{projected_lock}"
    );
    assert!(
        cargo_target.exists(),
        "successful native Rust build must produce the isolated target directory"
    );
}

#[test]
fn go_add_lib_runs_inside_mgc_without_spawning_go() {
    let project = TempDir::new().unwrap();
    go_lib_project(project.path());
    let sandbox = NoSpawnSandbox::multi(&["go"]);

    let (code, stdout, stderr) =
        sandbox.run_mgc(&["add-lib", "github.com/google/uuid"], project.path());
    let output = format!("{stdout}{stderr}");
    assert_eq!(
        code,
        Some(0),
        "native go `add-lib uuid` must succeed:\n{output}"
    );
    let gomod = std::fs::read_to_string(project.path().join("go.mod")).unwrap();
    assert!(
        gomod.contains("github.com/google/uuid"),
        "go.mod must require uuid (mgc-side edit):\n{gomod}"
    );
    assert!(
        sandbox.marker_text().is_empty(),
        "ZERO toolchain spawn allowed, got:\n{}",
        sandbox.marker_text()
    );
    let lock = std::fs::read_to_string(project.path().join("mgc.lock")).unwrap_or_default();
    assert!(
        lock.contains("github.com/google/uuid"),
        "mgc.lock must record uuid:\n{lock}"
    );
    assert!(
        lock.contains("sha256-") || lock.contains("sumdb-ziphash:h1:"),
        "mgc.lock must carry integrity evidence (proxy ziphash or sumdb record):\n{lock}"
    );
    let (code, stdout, stderr) = sandbox.run_mgc(&["install-lib"], project.path());
    let output = format!("{stdout}{stderr}");
    assert_eq!(
        code,
        Some(0),
        "native go `install-lib` must succeed:\n{output}"
    );
    assert!(
        sandbox.marker_text().is_empty(),
        "ZERO toolchain spawn during install, got:\n{}",
        sandbox.marker_text()
    );
}

fn dotnet_lib_project(dir: &std::path::Path) {
    std::fs::write(
        dir.join("mgc.toml"),
        "name = \"nativelib\"\necosystem = \"lib\"\n[lib]\nlanguage = \"dotnet\"\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("nativelib.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup>\n    <TargetFramework>net8.0</TargetFramework>\n  </PropertyGroup>\n</Project>\n",
    )
    .unwrap();
}

#[test]
fn dotnet_add_lib_runs_inside_mgc_without_spawning_dotnet() {
    let project = TempDir::new().unwrap();
    dotnet_lib_project(project.path());
    // No dotnet SDK on this machine on purpose: any toolchain spawn
    // would fail loudly instead of passing silently.
    let sandbox = NoSpawnSandbox::multi(&["dotnet"]);

    let (code, stdout, stderr) = sandbox.run_mgc(&["add-lib", "Newtonsoft.Json"], project.path());
    let output = format!("{stdout}{stderr}");
    assert_eq!(
        code,
        Some(0),
        "native dotnet `add-lib Newtonsoft.Json` must succeed:\n{output}"
    );
    let csproj = std::fs::read_to_string(project.path().join("nativelib.csproj")).unwrap();
    assert!(
        csproj.contains("Newtonsoft.Json"),
        "csproj must pin Newtonsoft.Json (mgc-side edit):\n{csproj}"
    );
    assert!(
        sandbox.marker_text().is_empty(),
        "ZERO toolchain spawn allowed, got:\n{}",
        sandbox.marker_text()
    );
    let lock = std::fs::read_to_string(project.path().join("mgc.lock")).unwrap_or_default();
    assert!(
        lock.contains("Newtonsoft.Json"),
        "mgc.lock must record Newtonsoft.Json:\n{lock}"
    );
    let (code, stdout, stderr) = sandbox.run_mgc(&["install-lib"], project.path());
    let output = format!("{stdout}{stderr}");
    assert_eq!(
        code,
        Some(0),
        "native dotnet `install-lib` must succeed:\n{output}"
    );
    assert!(
        sandbox.marker_text().is_empty(),
        "ZERO toolchain spawn during install, got:\n{}",
        sandbox.marker_text()
    );
}

fn java_lib_project(dir: &std::path::Path) {
    std::fs::write(
        dir.join("mgc.toml"),
        "name = \"nativelib\"\necosystem = \"lib\"\n[lib]\nlanguage = \"java\"\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("pom.xml"),
        "<project>\n  <modelVersion>4.0.0</modelVersion>\n  <groupId>com.example</groupId>\n  <artifactId>nativelib</artifactId>\n  <version>0.1.0</version>\n</project>\n",
    )
    .unwrap();
}

/// Local Maven metadata, POM, SHA-256, and JAR fixture for native Java E2E.
/// Fixture Maven cục bộ gồm metadata, POM, SHA-256 và JAR cho E2E Java native.
struct HermeticMaven {
    _server: mockito::ServerGuard,
    _mocks: Vec<mockito::Mock>,
    url: String,
}

impl HermeticMaven {
    fn new() -> Self {
        use sha2::{Digest, Sha256};

        let mut server = mockito::Server::new();
        let url = server.url();
        let jar = b"maven native add test artifact";
        let checksum = format!("{:x}", Sha256::digest(jar));
        let base = "/org/apache/commons/commons-lang3";
        let mocks = vec![
            server
                .mock("GET", format!("{base}/maven-metadata.xml").as_str())
                .expect_at_least(1)
                .with_status(200)
                .with_body(
                    "<metadata><versioning><versions><version>3.13.0</version></versions></versioning></metadata>",
                )
                .create(),
            server
                .mock("GET", format!("{base}/3.13.0/commons-lang3-3.13.0.pom").as_str())
                .expect_at_least(1)
                .with_status(200)
                .with_body(
                    "<project><modelVersion>4.0.0</modelVersion><groupId>org.apache.commons</groupId><artifactId>commons-lang3</artifactId><version>3.13.0</version></project>",
                )
                .create(),
            server
                .mock(
                    "GET",
                    format!("{base}/3.13.0/commons-lang3-3.13.0.jar.sha256").as_str(),
                )
                .expect_at_least(1)
                .with_status(200)
                .with_body(checksum)
                .create(),
            server
                .mock("GET", format!("{base}/3.13.0/commons-lang3-3.13.0.jar").as_str())
                .expect_at_least(1)
                .with_status(200)
                .with_body(jar.as_slice())
                .create(),
        ];
        Self {
            _server: server,
            _mocks: mocks,
            url,
        }
    }

    fn assert_all_routes_used(&self) {
        for mock in &self._mocks {
            mock.assert();
        }
    }
}

#[test]
fn java_add_lib_runs_inside_mgc_without_spawning_maven() {
    let project = TempDir::new().unwrap();
    java_lib_project(project.path());
    let registry = HermeticMaven::new();
    // mvn EXISTS on this machine: the canary shim shadows it, so any
    // spawn is recorded instead of running for real.
    let sandbox = NoSpawnSandbox::with_maven(&["mvn", "java", "gradle"], &registry.url);

    let (code, stdout, stderr) = sandbox.run_mgc(
        &["add-lib", "org.apache.commons:commons-lang3"],
        project.path(),
    );
    let output = format!("{stdout}{stderr}");
    assert_eq!(
        code,
        Some(0),
        "native java `add-lib commons-lang3` must succeed:\n{output}"
    );
    let pom = std::fs::read_to_string(project.path().join("pom.xml")).unwrap();
    assert!(
        pom.contains("commons-lang3"),
        "pom.xml must pin commons-lang3 (mgc-side edit):\n{pom}"
    );
    assert!(
        sandbox.marker_text().is_empty(),
        "ZERO toolchain spawn allowed, got:\n{}",
        sandbox.marker_text()
    );
    let lock = std::fs::read_to_string(project.path().join("mgc.lock")).unwrap_or_default();
    assert!(
        lock.contains("commons-lang3"),
        "mgc.lock must record commons-lang3:\n{lock}"
    );
    let (code, stdout, stderr) = sandbox.run_mgc(&["install-lib"], project.path());
    let output = format!("{stdout}{stderr}");
    assert_eq!(
        code,
        Some(0),
        "native java `install-lib` must succeed:\n{output}"
    );
    assert!(
        sandbox.marker_text().is_empty(),
        "ZERO toolchain spawn during install, got:\n{}",
        sandbox.marker_text()
    );
    registry.assert_all_routes_used();
}
