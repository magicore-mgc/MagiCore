//! `create-app flutter` without the Flutter SDK — create-app flutter không SDK.
//!
//! The app adapter installs Flutter dependencies with its own pub.dev engine, so
//! the SDK must not be a prerequisite for scaffolding. This case runs the real
//! binary with a PATH that cannot contain `flutter` and asserts the built-in
//! template wrote a manageable project. `mgc install` itself needs the network,
//! so only the manifest's shape is asserted here — the resolver's own unit
//! tests cover reading it.
//! Adapter app cài dependency Flutter bằng engine pub.dev riêng, nên SDK không
//! được là điều kiện để scaffold. Ca này chạy binary thật với PATH không thể có
//! `flutter` và kiểm template có sẵn đã ghi project quản lý được. `mgc install`
//! cần mạng nên ở đây chỉ kiểm hình dạng manifest — test unit của resolver làm
//! phần đọc nó.

#![allow(clippy::unwrap_used)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

/// PATH entries that cannot contain the Flutter SDK.
/// Các entry PATH không thể chứa SDK Flutter.
const SDK_FREE_PATH: &str = "/usr/bin:/bin";

fn mgc_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_mgc"))
}

fn flutter_on_path(path: &str) -> bool {
    path.split(':').any(|entry| {
        let candidate = Path::new(entry).join("flutter");
        candidate.is_file()
    })
}

#[test]
fn create_app_flutter_scaffolds_without_the_flutter_sdk() {
    // A host whose `/usr/bin:/bin` really holds flutter would invalidate the
    // premise; skip rather than assert something the environment cannot show.
    // (Máy mà `/usr/bin:/bin` thật sự có flutter sẽ phá tiền đề; bỏ qua thay vì
    // assert điều môi trường không thể chứng minh.)
    if flutter_on_path(SDK_FREE_PATH) {
        return;
    }

    let home = TempDir::new().unwrap();
    let workspace = TempDir::new().unwrap();
    let output = Command::new(mgc_binary())
        .current_dir(workspace.path())
        .env("HOME", home.path())
        .env("USERPROFILE", home.path())
        .env("MGC_DATA_DIR", home.path().join("data"))
        .env("MGC_CACHE_DIR", home.path().join("cache"))
        .env("MGC_TEMPLATES_DIR", home.path().join("templates"))
        .env("MAGICORE_STORE_ROOT", home.path().join("store"))
        .env("PATH", SDK_FREE_PATH)
        .env_remove("FLUTTER_ROOT")
        .args(["create-app", "flutter", "my_app", "--yes"])
        .output()
        .expect("spawn mgc create-app");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "create-app flutter must not need the SDK: {stderr}\n{stdout}"
    );

    let project = workspace.path().join("my_app");
    let pubspec = fs::read_to_string(project.join("pubspec.yaml")).expect("pubspec.yaml");
    assert!(pubspec.contains("name: my_app\n"), "{pubspec}");
    assert!(
        pubspec.contains("flutter: '"),
        "the manifest must declare a Flutter SDK constraint: {pubspec}"
    );
    assert!(
        !pubspec.contains("sdk: flutter"),
        "no SDK-sourced dependency may require the SDK at install time: {pubspec}"
    );

    let main = fs::read_to_string(project.join("lib/main.dart")).expect("lib/main.dart");
    assert!(main.contains("MaterialApp("), "{main}");
    assert!(project.join("analysis_options.yaml").is_file());
    assert!(project.join(".gitignore").is_file());
    assert_eq!(
        fs::read_to_string(project.join(".mgc.core"))
            .unwrap()
            .trim(),
        "app"
    );
    let manifest = fs::read_to_string(project.join("mgc.toml")).unwrap();
    assert!(manifest.contains("language = \"flutter\""), "{manifest}");
}
