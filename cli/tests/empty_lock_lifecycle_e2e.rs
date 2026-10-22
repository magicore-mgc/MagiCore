//! Empty native manifests still receive a schema-pinned lock for dev freshness.
//! Manifest native rỗng vẫn phải có lock ghim schema để dev freshness kiểm tra.

#![allow(clippy::unwrap_used)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

struct Fixture {
    _temp: TempDir,
    project: PathBuf,
    home: PathBuf,
}

impl Fixture {
    fn new(schema: &str) -> Self {
        let temp = TempDir::new().unwrap();
        let project = temp.path().join("project");
        let home = temp.path().join("home");
        std::fs::create_dir_all(&project).unwrap();
        std::fs::create_dir_all(home.join("data")).unwrap();
        std::fs::create_dir_all(home.join("cache")).unwrap();
        std::fs::create_dir_all(home.join("templates")).unwrap();
        std::fs::write(project.join(".mgc.core"), "web\n").unwrap();
        std::fs::write(
            project.join("mgc.toml"),
            format!(
                "name = \"empty-lock-fixture\"\nversion = \"0.1.0\"\necosystem = \"web\"\nframeworks = [\"vanilla\"]\n\n[lock]\nschema = \"{schema}\"\n"
            ),
        )
        .unwrap();
        std::fs::write(
            project.join("package.json"),
            "{\"name\":\"empty-lock-fixture\",\"version\":\"0.1.0\",\"dependencies\":{}}\n",
        )
        .unwrap();
        Self {
            _temp: temp,
            project,
            home,
        }
    }

    fn run(&self, args: &[&str]) -> Output {
        let binary = std::env::var_os("CARGO_BIN_EXE_mgc").expect("mgc test binary is set");
        let mut command = Command::new(binary);
        command
            .args(args)
            .current_dir(&self.project)
            .env("HOME", &self.home)
            .env("USERPROFILE", &self.home)
            .env("MGC_DATA_DIR", self.home.join("data"))
            .env("MGC_CACHE_DIR", self.home.join("cache"))
            .env("MGC_TEMPLATES_DIR", self.home.join("templates"))
            .env("MAGICORE_STORE_ROOT", self.home.join("store"))
            .env("CARGO_TARGET_DIR", self.project.join(".mgc-test-target"))
            .env("MGC_STRESS_TEST", "1")
            .env("CI", "1")
            .output()
            .unwrap()
    }

    fn new_lib_rust(schema: &str) -> Self {
        let temp = TempDir::new().unwrap();
        let project = temp.path().join("project");
        let home = temp.path().join("home");
        std::fs::create_dir_all(&project).unwrap();
        for directory in ["data", "cache", "templates"] {
            std::fs::create_dir_all(home.join(directory)).unwrap();
        }
        std::fs::write(project.join(".mgc.core"), "lib\n").unwrap();
        std::fs::write(
            project.join("mgc.toml"),
            format!(
                "name = \"empty-lib-fixture\"\nversion = \"0.1.0\"\necosystem = \"lib\"\nframeworks = [\"rust\"]\n\n[lock]\nschema = \"{schema}\"\n"
            ),
        )
        .unwrap();
        std::fs::write(
            project.join("Cargo.toml"),
            "[package]\nname = \"empty-lib-fixture\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        Self {
            _temp: temp,
            project,
            home,
        }
    }

    fn new_game_bevy(schema: &str) -> Self {
        let temp = TempDir::new().unwrap();
        let project = temp.path().join("project");
        let home = temp.path().join("home");
        std::fs::create_dir_all(&project).unwrap();
        for directory in ["data", "cache", "templates"] {
            std::fs::create_dir_all(home.join(directory)).unwrap();
        }
        std::fs::write(project.join(".mgc.core"), "game\n").unwrap();
        std::fs::write(
            project.join("mgc.toml"),
            format!(
                "name = \"empty-bevy-fixture\"\nversion = \"0.1.0\"\necosystem = \"game\"\nframeworks = [\"bevy\"]\n\n[game]\nengine = \"bevy\"\n\n[lock]\nschema = \"{schema}\"\n"
            ),
        )
        .unwrap();
        std::fs::write(
            project.join("Cargo.toml"),
            "[package]\nname = \"empty_bevy_fixture\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n",
        )
        .unwrap();
        std::fs::create_dir_all(project.join("src")).unwrap();
        std::fs::write(
            project.join("src/main.rs"),
            "fn main() { println!(\"empty Bevy lifecycle marker\"); }\n",
        )
        .unwrap();
        Self {
            _temp: temp,
            project,
            home,
        }
    }
}

fn assert_success(output: Output, context: &str) {
    assert!(
        output.status.success(),
        "{context} failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_lock_version(project: &Path, expected: u8) -> Vec<u8> {
    let path = project.join("mgc.lock");
    let bytes = std::fs::read(&path).expect("install must write mgc.lock for an empty manifest");
    let text = std::str::from_utf8(&bytes).unwrap();
    assert_eq!(
        mgc_lockfile::detect_lockfile_version(text).unwrap(),
        expected
    );
    bytes
}

#[test]
fn empty_web_install_writes_and_frozen_replays_v3_lock() {
    let fixture = Fixture::new("3");
    assert_success(
        fixture.run(&["install", "--core", "web"]),
        "empty V3 install",
    );
    let original = assert_lock_version(&fixture.project, 3);
    let lock = mgc_lockfile::parser::load_lockfile(&fixture.project.join("mgc.lock")).unwrap();
    assert!(lock.packages.is_empty());

    assert_success(
        fixture.run(&["install", "--frozen", "--core", "web"]),
        "empty V3 frozen install",
    );
    assert_eq!(
        std::fs::read(fixture.project.join("mgc.lock")).unwrap(),
        original,
        "frozen replay must preserve empty V3 lock bytes"
    );
}

#[test]
fn empty_web_install_writes_and_frozen_replays_v4_lock() {
    let fixture = Fixture::new("v4");
    assert_success(
        fixture.run(&["install", "--core", "web"]),
        "empty V4 install",
    );
    let original = assert_lock_version(&fixture.project, 4);
    let text = std::str::from_utf8(&original).unwrap();
    let document = mgc_lockfile::canonical::parse_v4_document(text).unwrap();
    let verification = mgc_lockfile::policy::verify_v4_math(&document).unwrap();
    assert!(verification.digest_ok, "empty V4 lock digest must verify");
    assert!(document.packages.is_empty());

    assert_success(
        fixture.run(&["install", "--frozen", "--core", "web"]),
        "empty V4 frozen install",
    );
    assert_eq!(
        std::fs::read(fixture.project.join("mgc.lock")).unwrap(),
        original,
        "frozen replay must preserve empty V4 lock bytes"
    );
}

#[test]
fn unversioned_web_application_installs_and_frozen_replays_without_manifest_mutation() {
    for (schema, version) in [("3", 3), ("v4", 4)] {
        let fixture = Fixture::new(schema);
        let package = fixture.project.join("package.json");
        let content = br#"{"name":"nuxt-style-app","private":true,"type":"module","dependencies":{},"scripts":{"dev":"nuxt dev"}}"#;
        std::fs::write(&package, content).expect("write unversioned application");
        assert_success(
            fixture.run(&["install", "--core", "web"]),
            "unversioned application install",
        );
        let original_lock = assert_lock_version(&fixture.project, version);
        assert_success(
            fixture.run(&["install", "--frozen", "--core", "web"]),
            "unversioned application frozen replay",
        );
        assert_eq!(
            std::fs::read(&package).expect("read application manifest"),
            content
        );
        assert_eq!(
            std::fs::read(fixture.project.join("mgc.lock")).expect("read lock"),
            original_lock
        );
    }
}

#[test]
fn empty_lib_rust_install_writes_and_frozen_replays_v3_and_v4_locks() {
    for schema in ["3", "v4"] {
        let fixture = Fixture::new_lib_rust(schema);
        assert_success(fixture.run(&["install-lib"]), "empty lib Rust install");
        let expected_version = if schema == "v4" { 4 } else { 3 };
        let original = assert_lock_version(&fixture.project, expected_version);
        let cargo_lock_path = fixture.project.join("Cargo.lock");
        let cargo_lock = std::fs::read(&cargo_lock_path)
            .expect("successful empty Rust install must write Cargo.lock");
        let cargo_document: toml::Value =
            toml::from_str(std::str::from_utf8(&cargo_lock).unwrap()).unwrap();
        assert_eq!(cargo_document["package"].as_array().unwrap().len(), 1);
        assert_eq!(
            cargo_document["package"][0]["name"].as_str(),
            Some("empty-lib-fixture")
        );

        assert_success(
            fixture.run(&["install-lib", "--frozen"]),
            "empty lib Rust frozen install",
        );
        assert_eq!(
            std::fs::read(fixture.project.join("mgc.lock")).unwrap(),
            original,
            "frozen replay must preserve empty lib lock bytes for schema {schema}"
        );
        assert_eq!(
            std::fs::read(cargo_lock_path).unwrap(),
            cargo_lock,
            "frozen replay must preserve the empty provider lock for schema {schema}"
        );
    }
}

#[test]
fn empty_bevy_install_writes_cargo_lock_and_dev_runs_v3_and_v4() {
    for schema in ["3", "v4"] {
        let fixture = Fixture::new_game_bevy(schema);
        assert_success(
            fixture.run(&["install-game", "--core", "game"]),
            "empty Bevy install",
        );

        let expected_version = if schema == "v4" { 4 } else { 3 };
        let canonical = assert_lock_version(&fixture.project, expected_version);
        if expected_version == 3 {
            assert!(
                mgc_lockfile::parser::load_lockfile(&fixture.project.join("mgc.lock"))
                    .unwrap()
                    .packages
                    .is_empty()
            );
        } else {
            let document = mgc_lockfile::canonical::parse_v4_document(
                std::str::from_utf8(&canonical).unwrap(),
            )
            .unwrap();
            assert!(
                mgc_lockfile::policy::verify_v4_math(&document)
                    .unwrap()
                    .digest_ok
            );
            assert!(document.packages.is_empty());
        }

        let cargo_lock_path = fixture.project.join("Cargo.lock");
        let cargo_lock_before = std::fs::read(&cargo_lock_path)
            .expect("successful empty Bevy install must write Cargo.lock");
        let cargo_lock: toml::Value =
            toml::from_str(std::str::from_utf8(&cargo_lock_before).unwrap()).unwrap();
        assert_eq!(cargo_lock["version"].as_integer(), Some(4));
        assert_eq!(cargo_lock["package"].as_array().unwrap().len(), 1);
        assert_eq!(
            cargo_lock["package"][0]["name"].as_str(),
            Some("empty_bevy_fixture")
        );
        assert_eq!(cargo_lock["package"][0]["version"].as_str(), Some("0.1.0"));

        let output = fixture.run(&["dev", "--core", "game"]);
        assert!(
            output.status.success(),
            "empty Bevy dev failed:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            String::from_utf8_lossy(&output.stdout).contains("empty Bevy lifecycle marker"),
            "mgc dev must execute the project binary:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
        assert_eq!(
            std::fs::read(cargo_lock_path).unwrap(),
            cargo_lock_before,
            "mgc dev must honor the provider lock without rewriting it"
        );
        assert_eq!(
            std::fs::read(fixture.project.join("mgc.lock")).unwrap(),
            canonical,
            "mgc dev must preserve the canonical MGC lock"
        );
    }
}

#[test]
fn empty_cargo_workspace_does_not_receive_a_partial_provider_lock() {
    for (workspace_field, expected_error) in [
        ("members = [\"member\"]", "workspace with members"),
        ("default-members = [\"member\"]", "workspace with members"),
        ("members = \"member\"", "workspace.members must be an array"),
    ] {
        let fixture = Fixture::new_game_bevy("3");
        std::fs::write(
            fixture.project.join("Cargo.toml"),
            format!(
                "[package]\nname = \"empty_bevy_fixture\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n{workspace_field}\n"
            ),
        )
        .unwrap();
        let member = fixture.project.join("member");
        std::fs::create_dir_all(member.join("src")).unwrap();
        std::fs::write(
            member.join("Cargo.toml"),
            "[package]\nname = \"workspace_member\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        std::fs::write(member.join("src/main.rs"), "fn main() {}\n").unwrap();

        let output = fixture.run(&["install-game", "--core", "game"]);
        assert!(
            !output.status.success(),
            "workspace install must fail closed"
        );
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(expected_error),
            "failure must explain why workspace Cargo.lock generation is unsupported:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            !fixture.project.join("Cargo.lock").exists(),
            "MGC must not publish an incomplete provider lock"
        );
        assert!(
            !fixture.project.join("mgc.lock").exists(),
            "MGC must reject an unsupported workspace before publishing its canonical lock"
        );
    }
}
