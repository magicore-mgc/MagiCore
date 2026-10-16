#![allow(clippy::unwrap_used)] // Test setup/assertions use unwrap; failure output identifies the fixture step.
// Setup/assertion dùng unwrap; khi lỗi, panic chỉ rõ bước fixture.

use serde_json::Value;
use std::path::Path;
use std::process::{Command, Output};
use tempfile::tempdir;

fn app_flutter_project(root: &Path) {
    std::fs::write(root.join(".mgc.core"), "app\n").unwrap();
    std::fs::write(
        root.join("mgc.toml"),
        "name = \"store-doctor-e2e\"\necosystem = \"app\"\nframeworks = [\"flutter\"]\n",
    )
    .unwrap();
}

fn run_mgc(project: &Path, home: &Path, data: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mgc"))
        .current_dir(project)
        .env("HOME", home)
        .env("USERPROFILE", home)
        .env("MGC_DATA_DIR", data)
        .env_remove("MAGICORE_STORE_ROOT")
        .env_remove("MGC_SWIFT_STORE_ROOT")
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn shared_store_doctor_reports_flutter_roots_read_only() {
    let project = tempdir().unwrap();
    let home = tempdir().unwrap();
    let data = tempdir().unwrap();
    app_flutter_project(project.path());

    let output = run_mgc(
        project.path(),
        home.path(),
        data.path(),
        &[
            "store",
            "doctor",
            "--core",
            "app",
            "--framework",
            "flutter",
            "--json",
        ],
    );
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["core"], "app");
    assert_eq!(report["framework"], "flutter");
    assert_eq!(report["scope"], "managed-root-path-safety-only");
    assert_eq!(report["integrity_checked"], false);
    assert_eq!(report["healthy"], true);
    assert_eq!(report["roots"].as_array().unwrap().len(), 2);
    let mut names = report["roots"]
        .as_array()
        .unwrap()
        .iter()
        .map(|root| root["name"].as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    names.sort();
    assert_eq!(names, ["content-cas", "pub"]);
    assert!(
        report["roots"]
            .as_array()
            .unwrap()
            .iter()
            .all(|root| root["safe"].as_bool() == Some(true))
    );
    assert!(
        !data.path().join("store").exists(),
        "doctor must not initialize global roots"
    );
}

#[test]
fn shared_store_doctor_resolves_each_native_global_store_root() {
    let home = tempdir().unwrap();
    let data = tempdir().unwrap();
    let routes = [
        ("ai", "python-agent", &["content-cas", "pypi"][..]),
        ("app", "flutter", &["content-cas", "pub"][..]),
        ("app", "swift", &["content-cas", "swift"][..]),
        ("game", "bevy", &["cargo", "content-cas"][..]),
        ("iot", "esp32-rust", &["cargo", "content-cas"][..]),
        ("lib", "dotnet", &["content-cas", "nuget"][..]),
        ("lib", "go", &["content-cas", "go"][..]),
        ("lib", "java", &["content-cas", "maven"][..]),
        ("lib", "python", &["content-cas", "pypi"][..]),
        ("lib", "rust", &["cargo", "content-cas"][..]),
        ("web", "actix-web", &["cargo", "content-cas"][..]),
        ("web", "axum", &["cargo", "content-cas"][..]),
        ("web", "rust", &["cargo", "content-cas"][..]),
        ("web", "django", &["content-cas", "pypi"][..]),
        ("web", "fastapi", &["content-cas", "pypi"][..]),
        ("web", "flask", &["content-cas", "pypi"][..]),
        ("web", "python", &["content-cas", "pypi"][..]),
        ("web", "echo", &["content-cas", "go"][..]),
        ("web", "fiber", &["content-cas", "go"][..]),
        ("web", "gin", &["content-cas", "go"][..]),
        ("web", "go", &["content-cas", "go"][..]),
        ("web", "java", &["content-cas", "maven"][..]),
        ("web", "quarkus", &["content-cas", "maven"][..]),
        ("web", "spring-boot", &["content-cas", "maven"][..]),
        ("web", "dotnet", &["content-cas", "nuget"][..]),
        ("web", "dotnet-minimal", &["content-cas", "nuget"][..]),
        ("web", "dotnet-webapi", &["content-cas", "nuget"][..]),
    ];

    for (core, framework, expected_names) in routes {
        let project = tempdir().unwrap();
        std::fs::write(project.path().join(".mgc.core"), format!("{core}\n")).unwrap();
        std::fs::write(
            project.path().join("mgc.toml"),
            format!(
                "name = \"store-doctor-e2e\"\necosystem = \"{core}\"\nframeworks = [\"{framework}\"]\n"
            ),
        )
        .unwrap();
        let output = run_mgc(
            project.path(),
            home.path(),
            data.path(),
            &[
                "store",
                "doctor",
                "--core",
                core,
                "--framework",
                framework,
                "--json",
            ],
        );
        assert!(
            output.status.success(),
            "{core}/{framework}: stdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let report: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["core"], core);
        assert_eq!(report["framework"], framework);
        assert_eq!(report["scope"], "managed-root-path-safety-only");
        let roots = report["roots"].as_array().unwrap();
        let mut actual_names = roots
            .iter()
            .map(|root| root["name"].as_str().unwrap().to_string())
            .collect::<Vec<_>>();
        let mut expected_names = expected_names
            .iter()
            .map(|name| (*name).to_string())
            .collect::<Vec<_>>();
        actual_names.sort();
        expected_names.sort();
        assert_eq!(actual_names, expected_names, "{core}/{framework}");
        for root in roots {
            let name = root["name"].as_str().unwrap();
            let expected_path = if name == "content-cas" {
                data.path().join("store/v3")
            } else {
                data.path().join("store").join(name)
            };
            assert_eq!(root["path"], expected_path.to_string_lossy().as_ref());
            assert_eq!(root["exists"], false);
            assert_eq!(root["safe"], true);
        }
    }
    assert!(
        !data.path().join("store").exists(),
        "doctor must stay read-only"
    );
}

#[test]
fn web_backed_native_routes_keep_the_web_integrity_check() {
    for (core, framework, ecosystem, declared_framework) in [
        ("clo", "cdk", "cloud", "cdk"),
        ("clo", "pulumi", "cloud", "pulumi"),
        ("lib", "ts", "lib", "ts"),
        ("web", "ts", "web", "vanilla"),
    ] {
        let project = tempdir().unwrap();
        let home = tempdir().unwrap();
        let data = tempdir().unwrap();
        std::fs::write(project.path().join(".mgc.core"), format!("{core}\n")).unwrap();
        std::fs::write(
            project.path().join("mgc.toml"),
            format!(
                "name = \"store-doctor-e2e\"\necosystem = \"{ecosystem}\"\nframeworks = [\"{declared_framework}\"]\n"
            ),
        )
        .unwrap();
        let web_store = project.path().join(".magicore/cache/web");
        std::fs::create_dir_all(&web_store).unwrap();
        mgc_store::Database::open(&web_store.join("store.db")).unwrap();

        let output = run_mgc(
            project.path(),
            home.path(),
            data.path(),
            &[
                "store",
                "doctor",
                "--core",
                core,
                "--framework",
                framework,
                "--json",
            ],
        );
        assert!(
            output.status.success(),
            "{core}/{framework}: stdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let report: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["core"], core);
        assert_eq!(report["framework"], framework);
        assert_eq!(report["scope"], "web-cas-and-generation-integrity");
        assert_eq!(report["integrity_checked"], true);
        assert_eq!(report["healthy"], true);
    }
}

#[test]
fn shared_store_doctor_refuses_unknown_framework_without_touching_roots() {
    let project = tempdir().unwrap();
    let home = tempdir().unwrap();
    let data = tempdir().unwrap();
    app_flutter_project(project.path());

    let output = run_mgc(
        project.path(),
        home.path(),
        data.path(),
        &[
            "store",
            "doctor",
            "--core",
            "app",
            "--framework",
            "maui",
            "--json",
        ],
    );
    assert!(!output.status.success());
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        combined.contains("no MagiCore-owned store route"),
        "{combined}"
    );
    assert!(
        !data.path().join("store").exists(),
        "unknown route must not touch stores"
    );
}

#[test]
fn shared_store_doctor_refuses_a_framework_not_declared_by_the_project() {
    let project = tempdir().unwrap();
    let home = tempdir().unwrap();
    let data = tempdir().unwrap();
    app_flutter_project(project.path());

    let output = run_mgc(
        project.path(),
        home.path(),
        data.path(),
        &[
            "store",
            "doctor",
            "--core",
            "app",
            "--framework",
            "swift",
            "--json",
        ],
    );
    assert!(!output.status.success());
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        combined.contains("mgc.toml declares 'flutter'"),
        "{combined}"
    );
    assert!(!data.path().join("store").exists());
}

#[test]
fn shared_root_repair_is_refused_without_mutation() {
    let project = tempdir().unwrap();
    let home = tempdir().unwrap();
    let data = tempdir().unwrap();
    app_flutter_project(project.path());

    let output = run_mgc(
        project.path(),
        home.path(),
        data.path(),
        &[
            "store",
            "doctor",
            "--core",
            "app",
            "--framework",
            "flutter",
            "--repair",
        ],
    );
    assert!(!output.status.success());
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(combined.contains("--repair is unsupported"), "{combined}");
    assert!(!data.path().join("store").exists());
}

#[test]
fn non_doctor_store_commands_remain_web_only() {
    let project = tempdir().unwrap();
    let home = tempdir().unwrap();
    let data = tempdir().unwrap();
    app_flutter_project(project.path());

    let output = run_mgc(
        project.path(),
        home.path(),
        data.path(),
        &["store", "status"],
    );
    assert!(!output.status.success());
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(combined.contains("only Web projects"), "{combined}");
    assert!(!data.path().join("store").exists());
}
