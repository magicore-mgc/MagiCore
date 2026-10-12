//! Native `install-ai` E2E: ai/python projects with a pyproject.toml MUST
//! install through MGC itself (parse → PyPI resolve → verified fetch →
//! mgc.lock + unpacked wheels) with ZERO uv/pip spawn. uv.lock-only
//! projects keep the explicit-compat legacy lane (out of scope here).
//! Local mock PyPI keeps the install evidence deterministic and network-independent.
//! (Mock PyPI cục bộ giúp bằng chứng install ổn định, không phụ thuộc mạng.)
//! (Install-ai native E2E: chạy trong MGC, không spawn uv/pip.)

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
    pypi_index_url: String,
}

impl NoSpawnSandbox {
    fn multi(tools: &[&str], pypi_index_url: &str) -> Self {
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
            pypi_index_url: pypi_index_url.to_string(),
        }
    }

    fn run_mgc(&self, args: &[&str], cwd: &std::path::Path) -> (Option<i32>, String, String) {
        let mut cmd = Command::new(mgc_binary());
        cmd.args(args)
            .current_dir(cwd)
            .env("PATH", self.path_env())
            .env("HOME", self.home_dir.path())
            .env("USERPROFILE", self.home_dir.path())
            .env("MGC_DATA_DIR", self.data_dir.path())
            .env("MGC_CACHE_DIR", self.data_dir.path().join("cache"))
            .env("MGC_TEMPLATES_DIR", self.data_dir.path().join("templates"))
            .env("MGC_PYPI_INDEX_URL", &self.pypi_index_url)
            .env_remove("MAGICORE_STORE_ROOT")
            .env_remove("MGC_COMPAT_RUNTIME")
            .env_remove("MGC_COMPAT_RUNNER");
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

// Build a valid minimal wheel so the E2E verifies parsing, hashing, fetching, and materialization offline.
// Tạo wheel tối giản hợp lệ để E2E xác minh parse, hash, tải và materialize mà không cần mạng ngoài.
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

// Keep PyPI metadata and wheel bytes local so CI does not depend on public registry availability.
// Dùng metadata PyPI và byte wheel cục bộ để CI không phụ thuộc registry công khai.
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

fn ai_python_project(dir: &std::path::Path) {
    std::fs::write(
        dir.join("mgc.toml"),
        "name = \"nativeagent\"\necosystem = \"ai\"\n[ai]\nframework = \"python-agent\"\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("pyproject.toml"),
        "[project]\nname = \"nativeagent\"\nversion = \"0.1.0\"\nrequires-python = \">=3.11\"\ndependencies = [\"six==1.17.0\"]\n",
    )
    .unwrap();
}

#[test]
fn ai_install_runs_inside_mgc_without_spawning_uv_or_pip() {
    let project = TempDir::new().unwrap();
    ai_python_project(project.path());
    let pypi = LocalPyPi::new();
    let sandbox =
        NoSpawnSandbox::multi(&["uv", "pip", "pip3", "python", "python3"], &pypi.index_url);

    let (code, stdout, stderr) = sandbox.run_mgc(&["install-ai"], project.path());
    let output = format!("{stdout}{stderr}");
    assert_eq!(
        code,
        Some(0),
        "native ai `install-ai` must succeed:\n{output}"
    );
    assert!(
        sandbox.marker_text().is_empty(),
        "ZERO toolchain spawn allowed, got:\n{}",
        sandbox.marker_text()
    );
    let lock = std::fs::read_to_string(project.path().join("mgc.lock")).unwrap_or_default();
    assert!(
        lock.contains("six") && lock.contains("sha256-"),
        "mgc.lock must record six with registry integrity (not uv.lock):\n{lock}"
    );
}

fn ai_empty_project(dir: &std::path::Path) {
    std::fs::write(
        dir.join("mgc.toml"),
        "name = \"nativeagent\"\necosystem = \"ai\"\n[ai]\nframework = \"python-agent\"\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("pyproject.toml"),
        "[project]\nname = \"nativeagent\"\nversion = \"0.1.0\"\nrequires-python = \">=3.11\"\ndependencies = []\n",
    )
    .unwrap();
}

#[test]
fn ai_v4_install_preserves_owner_integrity_and_frozen_lock_bytes() {
    let project = TempDir::new().unwrap();
    ai_python_project(project.path());
    let config_path = project.path().join("mgc.toml");
    let mut config = std::fs::read_to_string(&config_path).unwrap();
    config.push_str("\n[lock]\nschema = \"v4\"\n");
    let python = if cfg!(windows) { "python" } else { "python3" };
    config.push_str(&format!("\n[scripts]\ncheck = \"{python} check.py\"\n"));
    std::fs::write(config_path, config).unwrap();
    std::fs::write(
        project.path().join("check.py"),
        "from pathlib import Path\nimport six\nassert six.__version__ == '1.17.0'\nroot = Path(Path('expected-data-root.txt').read_text()).resolve()\nassert Path(six.__file__).resolve().is_relative_to(root), six.__file__\nprint('MGC_AI_V4_RUNTIME_OK')\n",
    )
    .unwrap();
    let pypi = LocalPyPi::new();
    let sandbox = NoSpawnSandbox::multi(&["uv", "pip", "pip3"], &pypi.index_url);
    // Child environment is sanitized; pass the expected storage root as fixture data.
    // Env của child được lọc; root mong đợi đi qua dữ liệu fixture, không nới whitelist.
    std::fs::write(
        project.path().join("expected-data-root.txt"),
        sandbox.data_dir.path().to_string_lossy().as_bytes(),
    )
    .unwrap();

    for args in [
        ["install-ai"].as_slice(),
        ["install-ai", "--frozen"].as_slice(),
    ] {
        let before = std::fs::read(project.path().join("mgc.lock")).ok();
        let (code, stdout, stderr) = sandbox.run_mgc(args, project.path());
        assert_eq!(code, Some(0), "AI V4 install failed:\n{stdout}{stderr}");
        let bytes = std::fs::read(project.path().join("mgc.lock")).unwrap();
        if let Some(before) = before {
            assert_eq!(
                bytes, before,
                "frozen AI V4 replay must preserve lock bytes"
            );
        }
        let text = std::str::from_utf8(&bytes).unwrap();
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
        assert!(document.root_dependencies_by_owner.contains_key("ai"));
        let package = document
            .packages
            .iter()
            .find(|row| row.key.name == "six")
            .unwrap();
        assert_eq!(package.key.version, "1.17.0");
        assert!(
            package
                .artifact
                .as_ref()
                .unwrap()
                .integrity_sri
                .as_ref()
                .unwrap()
                .starts_with("sha256-")
        );
    }
    assert!(
        sandbox.marker_text().is_empty(),
        "AI V4 must not spawn a package manager"
    );
    // Add a Lib-owned sibling with an intentionally unavailable artifact: AI must never fetch it.
    // Thêm sibling của Lib có artifact không khả dụng; replay AI phải không tải artifact đó.
    let path = project.path().join("mgc.lock");
    let mut document =
        mgc_lockfile::canonical::parse_v4_document(&std::fs::read_to_string(&path).unwrap())
            .unwrap();
    let roots = document.root_dependencies_by_owner.remove("ai").unwrap();
    document
        .root_dependencies_by_owner
        .insert("lib".into(), roots);
    let lib_owned = mgc_lockfile::writer::finalize_v4_document(&mut document).unwrap();
    std::fs::write(&path, &lib_owned).unwrap();
    let (code, stdout, stderr) = sandbox.run_mgc(&["install-ai", "--frozen"], project.path());
    assert_ne!(
        code,
        Some(0),
        "a missing AI owner cannot borrow Lib's frozen graph:\n{stdout}{stderr}"
    );
    assert_eq!(std::fs::read_to_string(&path).unwrap(), lib_owned);
    let (code, stdout, stderr) = sandbox.run_mgc(&["install-ai"], project.path());
    assert_eq!(
        code,
        Some(0),
        "fresh AI resolution must add its own owner without dropping Lib:\n{stdout}{stderr}"
    );
    let document =
        mgc_lockfile::canonical::parse_v4_document(&std::fs::read_to_string(&path).unwrap())
            .unwrap();
    let generator = document.metadata.generator.clone();
    let mut sibling = document.packages[0].clone();
    sibling.key.name = "sibling".into();
    sibling.artifact.as_mut().unwrap().url =
        format!("{}/files/unavailable-sibling.whl", pypi.index_url);
    let (mut shared, dropped) = mgc_lockfile::v4_graph::merge_v4_lane_packages(
        Some(document),
        vec![sibling],
        vec![],
        vec!["sibling@1.17.0".into()],
        "lib",
        &generator,
    );
    assert!(!dropped);
    let shared_bytes = mgc_lockfile::writer::finalize_v4_document(&mut shared).unwrap();
    std::fs::write(&path, &shared_bytes).unwrap();
    let (code, stdout, stderr) = sandbox.run_mgc(&["install-ai", "--frozen"], project.path());
    assert_eq!(
        code,
        Some(0),
        "shared V4 AI replay must ignore Lib artifacts:\n{stdout}{stderr}"
    );
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        shared_bytes,
        "shared V4 frozen replay must preserve both owners' lock bytes"
    );
    // Frozen must reject an orphan; a fresh resolve must repair it without losing sibling roots.
    // Frozen từ chối orphan; resolve mới phải sửa được và giữ roots của core khác.
    let mut orphan_document = shared.clone();
    let mut orphan = orphan_document.packages[0].clone();
    orphan.key.name = "unowned-orphan".into();
    orphan_document.packages.push(orphan);
    let orphan_bytes = mgc_lockfile::writer::finalize_v4_document(&mut orphan_document).unwrap();
    std::fs::write(&path, &orphan_bytes).unwrap();
    let (code, stdout, stderr) = sandbox.run_mgc(&["install-ai", "--frozen"], project.path());
    assert_ne!(code, Some(0), "frozen orphan must fail:\n{stdout}{stderr}");
    assert_eq!(std::fs::read_to_string(&path).unwrap(), orphan_bytes);
    let (code, stdout, stderr) = sandbox.run_mgc(&["install-ai"], project.path());
    assert_eq!(
        code,
        Some(0),
        "fresh orphan repair failed:\n{stdout}{stderr}"
    );
    let repaired =
        mgc_lockfile::canonical::parse_v4_document(&std::fs::read_to_string(&path).unwrap())
            .unwrap();
    assert_eq!(repaired.packages.len(), 2);
    assert_eq!(
        repaired.root_dependencies_by_owner,
        shared.root_dependencies_by_owner
    );
    let (code, stdout, stderr) = sandbox.run_mgc(&["run", "--core", "ai", "check"], project.path());
    assert_eq!(
        code,
        Some(0),
        "AI V4 Python runtime failed:\n{stdout}{stderr}"
    );
    assert!(
        stdout.contains("MGC_AI_V4_RUNTIME_OK"),
        "verified wheel must be importable:\n{stdout}{stderr}"
    );
    // Alter the artifact URL without refreshing the payload digest or breaking root identity.
    // Sửa URL artifact nhưng giữ root hợp lệ để kiểm riêng cổng xác minh digest.
    let lock_path = project.path().join("mgc.lock");
    let text = std::fs::read_to_string(&lock_path).unwrap();
    let mut document = mgc_lockfile::canonical::parse_v4_document(&text).unwrap();
    document.packages[0]
        .artifact
        .as_mut()
        .unwrap()
        .url
        .push_str("?tampered=1");
    let tampered = toml::to_string(&document).unwrap();
    std::fs::write(&lock_path, &tampered).unwrap();
    let (code, stdout, stderr) = sandbox.run_mgc(&["install-ai", "--frozen"], project.path());
    assert_ne!(
        code,
        Some(0),
        "tampered AI V4 lock must fail:\n{stdout}{stderr}"
    );
    assert!(
        format!("{stdout}{stderr}")
            .to_lowercase()
            .contains("digest"),
        "tampered artifact URL must be rejected by the digest gate:\n{stdout}{stderr}"
    );
    assert_eq!(
        std::fs::read_to_string(lock_path).unwrap(),
        tampered,
        "refused install must leave the tampered lock unchanged"
    );
    assert!(
        sandbox.marker_text().is_empty(),
        "refusal must not delegate to a package manager"
    );
}

#[test]
fn ai_add_runs_inside_mgc_without_spawning_uv_or_pip() {
    let project = TempDir::new().unwrap();
    ai_empty_project(project.path());
    let pypi = LocalPyPi::new();
    let sandbox =
        NoSpawnSandbox::multi(&["uv", "pip", "pip3", "python", "python3"], &pypi.index_url);

    let (code, stdout, stderr) = sandbox.run_mgc(&["add-ai", "six"], project.path());
    let output = format!("{stdout}{stderr}");
    assert_eq!(
        code,
        Some(0),
        "native ai `add-ai six` must succeed:\n{output}"
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
    let (code, stdout, stderr) = sandbox.run_mgc(&["install-ai"], project.path());
    let output = format!("{stdout}{stderr}");
    assert_eq!(
        code,
        Some(0),
        "native ai `install-ai` after add must succeed:\n{output}"
    );
    assert!(
        sandbox.marker_text().is_empty(),
        "ZERO toolchain spawn during install, got:\n{}",
        sandbox.marker_text()
    );
}

// Replay a verified wheel with no registry listener or metadata fetch.
// Replay wheel đã khóa từ cache, không cần listener hay metadata registry.
#[test]
fn ai_offline_reinstall_uses_verified_preseeded_wheel_without_registry() {
    for schema in ["3", "v4"] {
        let project = TempDir::new().unwrap();
        ai_python_project(project.path());
        let config_path = project.path().join("mgc.toml");
        let mut config = std::fs::read_to_string(&config_path).unwrap();
        config.push_str(&format!("\n[lock]\nschema = \"{schema}\"\n"));
        let python = if cfg!(target_os = "windows") {
            "python"
        } else {
            "python3"
        };
        config.push_str(&format!("\n[scripts]\ncheck = \"{python} check.py\"\n"));
        std::fs::write(project.path().join("check.py"), "from pathlib import Path\nimport six\nassert six.__version__ == '1.17.0'\nroot = Path('expected-data-root.txt').read_text()\nassert Path(six.__file__).resolve().is_relative_to(Path(root).resolve()), six.__file__\nprint('MGC_AI_OFFLINE_RUNTIME_OK')\n").unwrap();
        std::fs::write(config_path, config).unwrap();
        let sandbox = NoSpawnSandbox::multi(&["uv", "pip", "pip3"], "http://127.0.0.1:4315");
        let wheel = six_wheel();
        let digest = hex::encode(Sha256::digest(&wheel));
        let url = format!("https://files.pythonhosted.org/packages/{SIX_WHEEL_NAME}");
        let relative =
            mgc_resolver::protocols::PypiProtocol::artifact_cache_relpath(&url, &digest).unwrap();
        let cache = sandbox
            .data_dir
            .path()
            .join("store/pypi/wheels")
            .join(relative);
        std::fs::create_dir_all(cache.parent().unwrap()).unwrap();
        std::fs::write(&cache, &wheel).unwrap();
        let mut lock = mgc_lockfile::Lockfile::new();
        let mut package = mgc_lockfile::Package::new(
            "six".into(),
            "1.17.0".into(),
            url,
            format!("sha256-{digest}"),
        );
        package.ecosystem = mgc_lockfile::EcosystemTag::Python;
        package.owner_core = Some("ai".into());
        package.registry = Some("https://pypi.org".into());
        lock.add_package(package);
        mgc_lockfile::update_owner_root_pins(
            &mut lock,
            "ai",
            mgc_lockfile::EcosystemTag::Python,
            ["six@1.17.0".to_string()],
        );
        let bytes = if schema == "v4" {
            mgc_lockfile::migrate::finalize_migrated_v4(lock, "mgc-test", "2026-10-10T00:00:00Z")
                .unwrap()
        } else {
            mgc_lockfile::writer::serialize_lockfile(&lock).unwrap()
        };
        let lock_path = project.path().join("mgc.lock");
        std::fs::write(&lock_path, &bytes).unwrap();
        // Reject dependency drift before any cache artifact is materialized.
        // Chặn manifest lệch trước materialize; cache có đủ không cho phép bỏ freshness.
        let manifest_path = project.path().join("pyproject.toml");
        let manifest = std::fs::read_to_string(&manifest_path).unwrap();
        for stale in [
            manifest.replace("six==1.17.0", "six==1.18.0"),
            manifest.replace("six==1.17.0", "six==1.17.0\", \"unknown-package==1.0.0"),
            manifest.replace("dependencies = [\"six==1.17.0\"]", "dependencies = []"),
        ] {
            assert_ne!(stale, manifest, "drift fixture must alter dependencies");
            std::fs::write(&manifest_path, stale).unwrap();
            let (code, stdout, stderr) =
                sandbox.run_mgc(&["install", "--core", "ai", "--offline"], project.path());
            assert_ne!(code, Some(0), "stale manifest must fail: {stdout}{stderr}");
            assert_eq!(std::fs::read_to_string(&lock_path).unwrap(), bytes);
            assert_eq!(std::fs::read(&cache).unwrap(), wheel);
            assert!(
                !sandbox
                    .data_dir
                    .path()
                    .join("store/pypi/wheels/site")
                    .exists(),
                "drift must not materialize importable wheels"
            );
        }
        std::fs::write(&manifest_path, &manifest).unwrap();
        std::fs::remove_file(&lock_path).unwrap();
        let (code, stdout, stderr) = sandbox.run_mgc(&["install-ai", "--offline"], project.path());
        assert_ne!(code, Some(0), "missing lock must fail: {stdout}{stderr}");
        assert!(!lock_path.exists(), "offline cannot generate a new lock");
        std::fs::write(&lock_path, &bytes).unwrap();
        let (code, stdout, stderr) =
            sandbox.run_mgc(&["install", "--core", "ai", "--offline"], project.path());
        assert_eq!(
            code,
            Some(0),
            "AI cached offline reinstall failed:\n{stdout}{stderr}"
        );
        assert_eq!(std::fs::read_to_string(&lock_path).unwrap(), bytes);
        assert!(
            sandbox.marker_text().is_empty(),
            "offline reinstall spawned a package manager"
        );
        let (code, stdout, stderr) = sandbox.run_mgc(&["install-ai", "--offline"], project.path());
        assert_eq!(
            code,
            Some(0),
            "AI alias offline replay failed: {stdout}{stderr}"
        );
        std::fs::write(
            project.path().join("expected-data-root.txt"),
            sandbox.data_dir.path().to_string_lossy().as_bytes(),
        )
        .unwrap();
        let (code, stdout, stderr) =
            sandbox.run_mgc(&["run", "--core", "ai", "check"], project.path());
        assert_eq!(
            code,
            Some(0),
            "AI offline-installed module failed at runtime: {stdout}{stderr}"
        );
        assert!(
            stdout.contains("MGC_AI_OFFLINE_RUNTIME_OK"),
            "verified import marker missing: {stdout}{stderr}"
        );
        std::fs::remove_file(&cache).unwrap();
        let (code, stdout, stderr) = sandbox.run_mgc(&["install-ai", "--offline"], project.path());
        assert_ne!(code, Some(0), "missing wheel must fail: {stdout}{stderr}");
        assert!(
            format!("{stdout}{stderr}").contains("cache miss"),
            "missing-wheel error must explain cache miss: {stdout}{stderr}"
        );
        assert_eq!(std::fs::read_to_string(&lock_path).unwrap(), bytes);
        std::fs::write(&cache, b"tampered wheel").unwrap();
        let (code, stdout, stderr) = sandbox.run_mgc(&["install-ai", "--offline"], project.path());
        assert_ne!(code, Some(0), "tampered wheel must fail: {stdout}{stderr}");
        assert_eq!(std::fs::read_to_string(&lock_path).unwrap(), bytes);
        assert!(
            sandbox.marker_text().is_empty(),
            "negative replay spawned a PM"
        );
    }
}
