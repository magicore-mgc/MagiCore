#![allow(clippy::unwrap_used)]

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// SRI digest for a fixture tarball (SEC-01: integrity is mandatory).
/// (SRI của tarball fixture; SEC-01 bắt buộc integrity.)
fn sri_of(bytes: &[u8]) -> String {
    use base64::Engine as _;
    use sha2::{Digest, Sha512};
    format!(
        "sha512-{}",
        base64::engine::general_purpose::STANDARD.encode(Sha512::digest(bytes))
    )
}

use flate2::Compression;
use flate2::write::GzEncoder;
use tempfile::TempDir;

const PACKAGE: &str = "mgc-dlx-lifecycle-probe";
const VERSION: &str = "1.0.0";
const BINARY: &str = "mgc-dlx-lifecycle-probe";

struct RegistryFixture {
    _server: mockito::ServerGuard,
    _metadata: mockito::Mock,
    _tarball: mockito::Mock,
    url: String,
}

impl RegistryFixture {
    fn new() -> Self {
        let mut server = mockito::Server::new();
        let url = server.url();
        let mut metadata = serde_json::json!({
            "name": PACKAGE,
            "dist-tags": { "latest": VERSION },
            "versions": {
                (VERSION): {
                    "name": PACKAGE,
                    "version": VERSION,
                    "bin": { BINARY: "bin/cli" },
                    "scripts": { "postinstall": "node postinstall.cjs" },
                    "dependencies": {},
                    "dist": { "tarball": format!("{url}/{PACKAGE}/-/{PACKAGE}-{VERSION}.tgz"), "integrity": sri_of(&package_tarball()) }
                }
            }
        });
        // Seed publish times so the default 24h release-age quarantine does not
        // reject the fixture as unstamped; Dlx E2E exercises the lifecycle
        // script policy, not supply-chain ageing.
        // (Seed thời điểm publish để quarantine 24h mặc định không loại fixture
        // vì thiếu timestamp; test này kiểm policy lifecycle script, không phải
        // supply-chain ageing.)
        metadata["time"] = common::seeded_publish_times(&[VERSION]);
        let metadata_mock = server
            .mock("GET", format!("/{PACKAGE}").as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(metadata.to_string())
            .expect_at_least(1)
            .create();
        let tarball_mock = server
            .mock(
                "GET",
                format!("/{PACKAGE}/-/{PACKAGE}-{VERSION}.tgz").as_str(),
            )
            .with_status(200)
            .with_header("content-type", "application/octet-stream")
            .with_body(package_tarball())
            .expect_at_least(1)
            .create();
        Self {
            _server: server,
            _metadata: metadata_mock,
            _tarball: tarball_mock,
            url,
        }
    }
}

fn package_tarball() -> Vec<u8> {
    let manifest = serde_json::json!({
        "name": PACKAGE,
        "version": VERSION,
        "bin": { BINARY: "bin/cli" },
        "scripts": { "postinstall": "node postinstall.cjs" }
    })
    .to_string();
    let postinstall = "const fs = require('node:fs');\nconst path = require('node:path');\nfs.writeFileSync(path.join(__dirname, 'postinstall-ran'), 'yes');\n";
    let executable = "#!/usr/bin/env node\nconst fs = require('node:fs');\nconst path = require('node:path');\nconst ran = fs.existsSync(path.join(__dirname, '..', 'postinstall-ran'));\nconsole.log(ran ? 'MGC_DLX_POSTINSTALL_RAN' : 'MGC_DLX_POSTINSTALL_SKIPPED');\nconsole.log(`ARG:${process.argv[2] || ''}`);\n";
    let encoder = GzEncoder::new(Vec::new(), Compression::default());
    let mut archive = tar::Builder::new(encoder);
    append_file(
        &mut archive,
        "package/package.json",
        manifest.as_bytes(),
        0o644,
    );
    append_file(
        &mut archive,
        "package/postinstall.cjs",
        postinstall.as_bytes(),
        0o644,
    );
    append_file(
        &mut archive,
        "package/bin/cli",
        executable.as_bytes(),
        0o755,
    );
    archive.finish().unwrap();
    archive.into_inner().unwrap().finish().unwrap()
}

fn append_file(
    archive: &mut tar::Builder<GzEncoder<Vec<u8>>>,
    path: &str,
    content: &[u8],
    mode: u32,
) {
    let mut header = tar::Header::new_gnu();
    header.set_mode(mode);
    header.set_size(content.len() as u64);
    header.set_cksum();
    archive.append_data(&mut header, path, content).unwrap();
}

fn mgc_binary() -> PathBuf {
    if let Some(value) = std::env::var_os("MGC_TEST_CLI_BINARY") {
        let path = PathBuf::from(value);
        if path.is_file() {
            eprintln!("MagiCore dlx product binary: {}", path.display());
            return path;
        }
        #[cfg(windows)]
        {
            let executable = path.with_extension("exe");
            if executable.is_file() {
                eprintln!("MagiCore dlx product binary: {}", executable.display());
                return executable;
            }
        }
        panic!(
            "MGC_TEST_CLI_BINARY does not identify an executable: {}",
            path.display()
        );
    }
    let binary = PathBuf::from(env!("CARGO_BIN_EXE_mgc"));
    eprintln!("MagiCore dlx test binary: {}", binary.display());
    binary
}

fn run_dlx(
    binary: &Path,
    args: &[&str],
    cwd: &Path,
    home: &Path,
    data: &Path,
    registry: &str,
) -> Output {
    Command::new(binary)
        .args(args)
        .current_dir(cwd)
        .env_remove("MGC_TEST_CLI_BINARY")
        .env("HOME", home)
        .env("USERPROFILE", home)
        .env("MGC_DATA_DIR", data)
        .env("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST", "1")
        .env("MAGICORE_WEB_REGISTRY_URL", registry)
        .env("MAGICORE_WEB_ALLOWED_REGISTRIES", registry)
        .env("CARGO_NET_OFFLINE", "true")
        .output()
        .expect("spawn MagiCore dlx product binary")
}

#[test]
#[ignore = "requires Node.js; exercised with setup-node in the lifecycle matrix"]
fn dlx_runs_lifecycle_scripts_only_after_explicit_opt_in() {
    let registry = RegistryFixture::new();
    let sandbox = TempDir::new().unwrap();
    let caller = sandbox.path().join("caller");
    let home = sandbox.path().join("home");
    let data = sandbox.path().join("data");
    std::fs::create_dir_all(&caller).unwrap();
    std::fs::create_dir_all(&home).unwrap();
    let binary = mgc_binary();

    let default = run_dlx(
        &binary,
        &["dlx", &format!("{PACKAGE}@{VERSION}"), "--", "probe-arg"],
        &caller,
        &home,
        &data,
        &registry.url,
    );
    let default_output = format!(
        "{}{}",
        String::from_utf8_lossy(&default.stdout),
        String::from_utf8_lossy(&default.stderr)
    );
    assert!(
        default.status.success(),
        "default dlx execution failed:\n{default_output}"
    );
    assert!(default_output.contains("MGC_DLX_POSTINSTALL_SKIPPED"));
    assert!(default_output.contains("ARG:probe-arg"));

    let allowed = run_dlx(
        &binary,
        &[
            "dlx",
            &format!("{PACKAGE}@{VERSION}"),
            "--allow-scripts",
            "--",
            "probe-arg",
        ],
        &caller,
        &home,
        &data,
        &registry.url,
    );
    let allowed_output = format!(
        "{}{}",
        String::from_utf8_lossy(&allowed.stdout),
        String::from_utf8_lossy(&allowed.stderr)
    );
    assert!(
        allowed.status.success(),
        "explicit lifecycle-script dlx execution failed:\n{allowed_output}"
    );
    assert!(allowed_output.contains("MGC_DLX_POSTINSTALL_RAN"));
    assert!(allowed_output.contains("lifecycle scripts are enabled"));
    assert!(allowed_output.contains("ARG:probe-arg"));
}
