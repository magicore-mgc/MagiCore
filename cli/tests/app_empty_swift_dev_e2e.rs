//! Empty Swift app projects must pass the strict dev freshness pre-flight
//! immediately after a successful MGC install, for both supported lock schemas.
//! (Project app Swift không dependency phải qua freshness ngay sau mgc install,
//! với cả hai schema lock đang hỗ trợ.)

#![cfg(target_os = "macos")]
#![allow(clippy::unwrap_used)]

use std::process::Command;
use tempfile::TempDir;

fn write_swift_app(root: &std::path::Path, schema: &str) {
    std::fs::write(
        root.join("mgc.toml"),
        format!(
            "name = \"app-probe\"\necosystem = \"app\"\n[app]\nlanguage = \"swift\"\n[lock]\nschema = \"{schema}\"\n"
        ),
    )
    .unwrap();
    std::fs::write(
        root.join("Package.swift"),
        "// swift-tools-version: 5.9\nimport PackageDescription\nlet package = Package(name: \"AppProbe\", products: [.executable(name: \"AppProbe\", targets: [\"AppProbe\"])], dependencies: [], targets: [.executableTarget(name: \"AppProbe\")])\n",
    )
    .unwrap();
    let source = root.join("Sources/AppProbe");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::write(
        source.join("main.swift"),
        "print(\"MGC_SWIFT_APP_DEV_OK\")\n",
    )
    .unwrap();
}

fn write_flutter_app(root: &std::path::Path, schema: &str) {
    std::fs::write(
        root.join("mgc.toml"),
        format!(
            "name = \"app-probe\"\necosystem = \"app\"\n[app]\nlanguage = \"flutter\"\n[lock]\nschema = \"{schema}\"\n"
        ),
    )
    .unwrap();
    std::fs::write(
        root.join("pubspec.yaml"),
        "name: app_probe\nversion: 1.0.0\nenvironment:\n  sdk: '>=3.0.0 <4.0.0'\ndependencies: {}\ndev_dependencies: {}\n",
    )
    .unwrap();
}

#[test]
fn empty_native_app_install_writes_lock_and_swift_dev_runs_for_v3_and_v4() {
    for schema in ["3", "v4"] {
        let project = TempDir::new().unwrap();
        let home = TempDir::new().unwrap();
        let data = TempDir::new().unwrap();
        write_swift_app(project.path(), schema);

        let install = Command::new(env!("CARGO_BIN_EXE_mgc"))
            .args(["install-app", "--core", "app"])
            .current_dir(project.path())
            .env("HOME", home.path())
            .env("USERPROFILE", home.path())
            .env("MGC_DATA_DIR", data.path())
            .output()
            .expect("spawn mgc install-app");
        assert!(
            install.status.success(),
            "mgc install-app must succeed for the empty Swift fixture (schema {schema}); stdout={} stderr={}",
            String::from_utf8_lossy(&install.stdout),
            String::from_utf8_lossy(&install.stderr)
        );
        assert!(
            project.path().join("mgc.lock").is_file(),
            "successful empty Swift install must write mgc.lock before dev freshness checks (schema {schema})"
        );

        let dev = Command::new(env!("CARGO_BIN_EXE_mgc"))
            .args(["dev", "--core", "app"])
            .current_dir(project.path())
            .env("HOME", home.path())
            .env("USERPROFILE", home.path())
            .env("MGC_DATA_DIR", data.path())
            .output()
            .expect("spawn mgc dev app");
        assert!(
            dev.status.success(),
            "mgc dev app must run after successful empty Swift install (schema {schema}); stdout={} stderr={}",
            String::from_utf8_lossy(&dev.stdout),
            String::from_utf8_lossy(&dev.stderr)
        );
        assert!(
            String::from_utf8_lossy(&dev.stdout).contains("MGC_SWIFT_APP_DEV_OK"),
            "mgc dev app must execute the Swift fixture (schema {schema}); stdout={} stderr={}",
            String::from_utf8_lossy(&dev.stdout),
            String::from_utf8_lossy(&dev.stderr)
        );
        assert!(
            String::from_utf8_lossy(&dev.stdout).contains("target: SwiftPM CLI"),
            "SwiftPM projects must report the target they actually run; stdout={} stderr={}",
            String::from_utf8_lossy(&dev.stdout),
            String::from_utf8_lossy(&dev.stderr)
        );

        let flutter_project = TempDir::new().unwrap();
        write_flutter_app(flutter_project.path(), schema);
        let flutter_install = Command::new(env!("CARGO_BIN_EXE_mgc"))
            .args(["install-app", "--core", "app"])
            .current_dir(flutter_project.path())
            .env("HOME", home.path())
            .env("USERPROFILE", home.path())
            .env("MGC_DATA_DIR", data.path())
            .output()
            .expect("spawn mgc install-app for Flutter");
        assert!(
            flutter_install.status.success(),
            "mgc install-app must succeed for empty Flutter fixture (schema {schema}); stdout={} stderr={}",
            String::from_utf8_lossy(&flutter_install.stdout),
            String::from_utf8_lossy(&flutter_install.stderr)
        );
        assert!(
            flutter_project.path().join("mgc.lock").is_file(),
            "successful empty Flutter install must write mgc.lock before dev freshness checks (schema {schema})"
        );
    }
}
