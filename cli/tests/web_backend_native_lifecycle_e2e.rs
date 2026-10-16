//! Non-JS web backend dependency operations must retain web project identity
//! while using the language-native resolver owned by the lib engine.
//! Backend web ngoài JS phải giữ identity web khi dùng resolver theo ngôn ngữ.

#![allow(clippy::unwrap_used)]

use std::process::Command;
use tempfile::TempDir;

struct BackendFixture {
    project: TempDir,
    home: TempDir,
    data: TempDir,
}

fn write_backend_fixture(framework: &str, language: &str, schema: &str) -> BackendFixture {
    let project = TempDir::new().expect("create backend project");
    let home = TempDir::new().expect("create isolated home");
    let data = TempDir::new().expect("create isolated system data");
    let root = project.path();

    let mut config = mgc_config::project::ProjectConfig::new("backend-probe", "web");
    config.mode = "backend".to_string();
    config.frameworks = vec![language.to_string(), framework.to_string()];
    config.lock = Some(mgc_config::project::LockConfig {
        schema: Some(schema.to_string()),
        ..Default::default()
    });
    config
        .save(root)
        .expect("write web project config and identity");

    match language {
        "python" => std::fs::write(
            root.join("pyproject.toml"),
            "[project]\nname = \"backend-probe\"\nversion = \"0.1.0\"\ndependencies = []\n",
        )
        .unwrap(),
        "go" => std::fs::write(
            root.join("go.mod"),
            "module example.com/backend-probe\n\ngo 1.24\n",
        )
        .unwrap(),
        "rust" => std::fs::write(
            root.join("Cargo.toml"),
            "[package]\nname = \"backend-probe\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n",
        )
        .unwrap(),
        "java" => std::fs::write(
            root.join("pom.xml"),
            "<project><modelVersion>4.0.0</modelVersion><groupId>com.example</groupId><artifactId>backend-probe</artifactId><version>0.1.0</version></project>",
        )
        .unwrap(),
        "dotnet" => std::fs::write(
            root.join("BackendProbe.csproj"),
            "<Project Sdk=\"Microsoft.NET.Sdk\"><PropertyGroup><TargetFramework>net8.0</TargetFramework></PropertyGroup></Project>",
        )
        .unwrap(),
        other => panic!("unsupported test language: {other}"),
    }

    BackendFixture {
        project,
        home,
        data,
    }
}

fn run_mgc(fixture: &BackendFixture, args: &[&str]) -> (bool, String) {
    let binary = std::env::var_os("MGC_TEST_CLI_BINARY")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from(env!("CARGO_BIN_EXE_mgc")));
    assert!(
        binary.is_file(),
        "mgc executable missing: {}",
        binary.display()
    );
    eprintln!("MagiCore Web backend product binary: {}", binary.display());
    let output = Command::new(binary)
        .args(args)
        .current_dir(fixture.project.path())
        .env("HOME", fixture.home.path())
        .env("USERPROFILE", fixture.home.path())
        .env("MGC_DATA_DIR", fixture.data.path())
        .env("MGC_CACHE_DIR", fixture.data.path().join("cache"))
        .env("MGC_TEMPLATES_DIR", fixture.data.path().join("templates"))
        .output()
        .expect("spawn mgc");
    (
        output.status.success(),
        format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
    )
}

#[test]
fn empty_non_js_web_backends_install_and_frozen_replay_v3_and_v4() {
    for (framework, language) in [
        ("actix-web", "rust"),
        ("axum", "rust"),
        ("django", "python"),
        ("fastapi", "python"),
        ("flask", "python"),
        ("echo", "go"),
        ("fiber", "go"),
        ("gin", "go"),
        ("quarkus", "java"),
        ("spring-boot", "java"),
        ("dotnet-minimal", "dotnet"),
        ("dotnet-webapi", "dotnet"),
        ("rust", "rust"),
        ("python", "python"),
        ("go", "go"),
        ("java", "java"),
        ("dotnet", "dotnet"),
    ] {
        for (schema, expected_version) in [("3", 3), ("v4", 4)] {
            let fixture = write_backend_fixture(framework, language, schema);
            let (ok, output) = run_mgc(&fixture, &["install-web"]);
            assert!(
                ok,
                "{framework}/{language} install-web failed (schema {schema}):\n{output}"
            );
            let core_marker = std::fs::read_to_string(fixture.project.path().join(".mgc.core"))
                .unwrap_or_else(|error| {
                    panic!("{framework}/{language} install-web removed .mgc.core: {error}")
                });
            assert_eq!(
                core_marker.trim(),
                "web",
                "{framework}/{language} changed the owning project core"
            );

            let lock = std::fs::read_to_string(fixture.project.path().join("mgc.lock"))
                .unwrap_or_else(|error| {
                    panic!("{framework}/{language} install-web did not write mgc.lock: {error}")
                });
            let actual_version = mgc_lockfile::migrate::detect_lockfile_version(&lock)
                .expect("parse generated lock schema");
            assert_eq!(
                actual_version, expected_version,
                "{framework}/{language} wrote the wrong lock schema"
            );

            let (ok, output) = run_mgc(&fixture, &["install-web", "--frozen"]);
            assert!(
                ok,
                "{framework}/{language} frozen install-web failed (schema {schema}):\n{output}"
            );
            let core_marker_after_replay = std::fs::read_to_string(
                fixture.project.path().join(".mgc.core"),
            )
            .unwrap_or_else(|error| {
                panic!("{framework}/{language} frozen install removed .mgc.core: {error}")
            });
            assert_eq!(
                core_marker_after_replay.trim(),
                "web",
                "{framework}/{language} frozen replay changed the owning project core"
            );
        }
    }
}

#[test]
fn web_backend_gate_uses_the_selected_language_with_multiple_manifests() {
    let fixture = write_backend_fixture("fastapi", "python", "3");
    std::fs::write(
        fixture.project.path().join("pom.xml"),
        "<project><modelVersion>4.0.0</modelVersion><groupId>com.example</groupId><artifactId>backend-probe</artifactId><version>0.1.0</version></project>",
    )
    .expect("write secondary Java manifest");

    let (ok, output) = run_mgc(&fixture, &["list-web"]);
    assert!(
        ok,
        "web selected the Python backend, so the dependency gate must use Python too:\n{output}"
    );
}
