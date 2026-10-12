//! CLI E2E for the explicit V4 web lock lifecycle using a local npm fixture.
//! E2E CLI cho vòng đời lock web V4 tường minh với registry npm cục bộ.

#![allow(clippy::unwrap_used)]

use base64::Engine as _;
use flate2::Compression;
use flate2::write::GzEncoder;
use sha2::{Digest, Sha512};
use std::process::{Command, Output};
use tempfile::TempDir;

struct NpmFixture {
    _server: mockito::ServerGuard,
    _mocks: Vec<mockito::Mock>,
    url: String,
    integrity: String,
}

impl NpmFixture {
    fn new() -> Self {
        let mut server = mockito::Server::new();
        let url = server.url();
        let tarball = npm_tarball();
        let integrity = format!(
            "sha512-{}",
            base64::engine::general_purpose::STANDARD.encode(Sha512::digest(&tarball))
        );
        let metadata = serde_json::json!({
            "name": "is-odd",
            "dist-tags": {"latest": "3.0.1"},
            "versions": {
                "3.0.1": {
                    "name": "is-odd",
                    "version": "3.0.1",
                    "dependencies": {},
                    "dist": {
                        "integrity": integrity,
                        "tarball": format!("{url}/is-odd/-/is-odd-3.0.1.tgz")
                    }
                }
            }
        });
        let mocks = vec![
            server
                .mock("GET", "/is-odd")
                .expect_at_least(1)
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(metadata.to_string())
                .create(),
            server
                .mock("GET", "/is-odd/-/is-odd-3.0.1.tgz")
                .expect_at_least(1)
                .with_status(200)
                .with_header("content-type", "application/octet-stream")
                .with_body(tarball)
                .create(),
        ];
        Self {
            _server: server,
            _mocks: mocks,
            url,
            integrity,
        }
    }
}

fn npm_tarball() -> Vec<u8> {
    let encoder = GzEncoder::new(Vec::new(), Compression::default());
    let mut archive = tar::Builder::new(encoder);
    for (path, contents) in [
        (
            "package/package.json",
            r#"{"name":"is-odd","version":"3.0.1","main":"index.js"}"#,
        ),
        (
            "package/index.js",
            "module.exports = n => Math.abs(n % 2) === 1;",
        ),
    ] {
        let mut header = tar::Header::new_gnu();
        header.set_size(contents.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        archive
            .append_data(&mut header, path, contents.as_bytes())
            .unwrap();
    }
    archive.into_inner().unwrap().finish().unwrap()
}

fn create_v4_project(project: &std::path::Path) {
    std::fs::create_dir_all(project).unwrap();
    std::fs::write(
        project.join("package.json"),
        r#"{"name":"v4-cli-fixture","version":"1.0.0","dependencies":{}}"#,
    )
    .unwrap();
    std::fs::write(
        project.join("mgc.toml"),
        "name = \"v4-cli-fixture\"\nversion = \"1.0.0\"\necosystem = \"web\"\n[lock]\nschema = \"v4\"\n",
    )
    .unwrap();
    std::fs::write(project.join(".mgc.core"), "web\n").unwrap();
}

fn run_mgc(
    project: &std::path::Path,
    home: &std::path::Path,
    registry_url: &str,
    args: &[&str],
) -> Output {
    let mut command = Command::new(
        std::env::var_os("CARGO_BIN_EXE_mgc").expect("CARGO_BIN_EXE_mgc must be set by cargo test"),
    );
    command
        .args(args)
        .current_dir(project)
        .env("HOME", home)
        .env("USERPROFILE", home)
        .env("MGC_DATA_DIR", home.join("system-data"))
        .env("MGC_CACHE_DIR", project.join(".mgc-cache"))
        .env("MGC_TEMPLATES_DIR", home.join("templates"))
        .env("MAGICORE_STORE_ROOT", home.join("store"))
        .env("MAGICORE_WEB_REGISTRY_URL", registry_url)
        .env("MAGICORE_WEB_ALLOWED_REGISTRIES", registry_url)
        .env("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST", "1")
        .env_remove("MGC_COMPAT_RUNTIME")
        .env_remove("MGC_COMPAT_RUNNER");
    command.output().expect("spawn the just-built mgc binary")
}

fn assert_v4_lock(project: &std::path::Path, expected_packages: usize) -> String {
    let path = project.join("mgc.lock");
    let text = std::fs::read_to_string(&path).expect("mgc.lock must be written");
    assert_eq!(
        mgc_lockfile::migrate::detect_lockfile_version(&text).unwrap(),
        4,
        "explicit [lock] schema = v4 must write schema 4"
    );
    let document = mgc_lockfile::canonical::parse_v4_document(&text).unwrap();
    let verification = mgc_lockfile::policy::verify_v4_math(&document).unwrap();
    assert!(verification.digest_ok, "v4 payload digest must verify");
    assert_eq!(document.packages.len(), expected_packages);
    text
}

#[test]
fn explicit_v4_schema_survives_add_frozen_replay_update_and_remove() {
    let fixture = NpmFixture::new();
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("project");
    let home = temp.path().join("home");
    std::fs::create_dir_all(&home).unwrap();
    create_v4_project(&project);

    let add = run_mgc(
        &project,
        &home,
        &fixture.url,
        &["--core", "web", "add", "is-odd@3.0.1"],
    );
    assert!(
        add.status.success(),
        "native add into V4 project failed:\n{}{}",
        String::from_utf8_lossy(&add.stdout),
        String::from_utf8_lossy(&add.stderr)
    );
    let after_add = assert_v4_lock(&project, 1);
    let document = mgc_lockfile::canonical::parse_v4_document(&after_add).unwrap();
    let package = document
        .packages
        .iter()
        .find(|package| package.key.name == "is-odd")
        .expect("V4 lock must preserve the resolved package identity");
    assert_eq!(
        package
            .artifact
            .as_ref()
            .and_then(|artifact| artifact.integrity_sri.as_deref()),
        Some(fixture.integrity.as_str()),
        "V4 install must retain the registry SRI used to verify artifact bytes"
    );

    let frozen = run_mgc(
        &project,
        &home,
        &fixture.url,
        &["--core", "web", "install", "--frozen"],
    );
    assert!(
        frozen.status.success(),
        "frozen V4 replay failed:\n{}{}",
        String::from_utf8_lossy(&frozen.stdout),
        String::from_utf8_lossy(&frozen.stderr)
    );
    assert_eq!(
        std::fs::read(project.join("mgc.lock")).unwrap(),
        after_add.as_bytes(),
        "frozen replay must preserve V4 lock bytes"
    );

    let update = run_mgc(
        &project,
        &home,
        &fixture.url,
        &["--core", "web", "update", "is-odd"],
    );
    assert!(
        update.status.success(),
        "V4 update failed:\n{}{}",
        String::from_utf8_lossy(&update.stdout),
        String::from_utf8_lossy(&update.stderr)
    );
    assert_v4_lock(&project, 1);

    let remove = run_mgc(
        &project,
        &home,
        &fixture.url,
        &["--core", "web", "remove", "is-odd"],
    );
    assert!(
        remove.status.success(),
        "V4 remove failed:\n{}{}",
        String::from_utf8_lossy(&remove.stdout),
        String::from_utf8_lossy(&remove.stderr)
    );
    assert_v4_lock(&project, 0);
}

#[test]
fn explicit_v4_schema_writes_and_replays_empty_graph_via_install_web() {
    let fixture = NpmFixture::new();
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("project");
    let home = temp.path().join("home");
    std::fs::create_dir_all(&home).unwrap();
    create_v4_project(&project);
    std::fs::write(
        project.join("mgc.toml"),
        "name = \"v4-cli-fixture\"\nversion = \"1.0.0\"\necosystem = \"web\"\n[lock]\nschema = \"3\"\n",
    )
    .unwrap();

    let legacy_install = run_mgc(&project, &home, &fixture.url, &["install-web"]);
    assert!(
        legacy_install.status.success(),
        "empty native V3 install-web failed:\n{}{}",
        String::from_utf8_lossy(&legacy_install.stdout),
        String::from_utf8_lossy(&legacy_install.stderr)
    );
    let legacy_text = std::fs::read_to_string(project.join("mgc.lock")).unwrap();
    assert_eq!(
        mgc_lockfile::migrate::detect_lockfile_version(&legacy_text).unwrap(),
        3,
        "initial dependency-free install must establish the legacy V3 lock"
    );

    std::fs::write(
        project.join("mgc.toml"),
        "name = \"v4-cli-fixture\"\nversion = \"1.0.0\"\necosystem = \"web\"\n[lock]\nschema = \"v4\"\n",
    )
    .unwrap();
    let install = run_mgc(&project, &home, &fixture.url, &["install-web"]);
    assert!(
        install.status.success(),
        "empty native V4 install-web failed:\n{}{}",
        String::from_utf8_lossy(&install.stdout),
        String::from_utf8_lossy(&install.stderr)
    );
    let initial_v4 = assert_v4_lock(&project, 0);

    let frozen = run_mgc(&project, &home, &fixture.url, &["install-web", "--frozen"]);
    assert!(
        frozen.status.success(),
        "empty V4 frozen install-web replay failed:\n{}{}",
        String::from_utf8_lossy(&frozen.stdout),
        String::from_utf8_lossy(&frozen.stderr)
    );
    assert_eq!(
        std::fs::read(project.join("mgc.lock")).unwrap(),
        initial_v4.as_bytes(),
        "empty V4 replay must preserve canonical lock bytes"
    );
}
