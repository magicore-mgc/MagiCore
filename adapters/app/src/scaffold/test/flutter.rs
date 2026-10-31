//! Offline Flutter template tests — kiểm template Flutter offline.
//!
//! A green run proves `create-app flutter` can scaffold on a machine with no
//! Flutter SDK: the manifest is a valid pubspec, the entry point is a real
//! `MaterialApp`, and the project is detectable by the adapter's own language
//! detection — the same predicate `mgc install` uses to pick the pub.dev lane.
//! Test xanh chứng minh `create-app flutter` scaffold được trên máy không có SDK
//! Flutter: manifest là pubspec hợp lệ, entry point là `MaterialApp` thật, và
//! project được chính predicate nhận diện language của adapter công nhận —
//! predicate mà `mgc install` dùng để chọn lane pub.dev.

use std::path::{Path, PathBuf};

use super::super::{DART_SDK_CONSTRAINT, FLUTTER_SDK_CONSTRAINT, package_name, scaffold_project};

#[test]
fn flutter_scaffold_writes_a_manageable_project() {
    let dir = target("basic");
    runtime()
        .block_on(scaffold_project("my_app", &dir))
        .expect("offline flutter scaffold");

    let pubspec = std::fs::read_to_string(dir.join("pubspec.yaml")).expect("pubspec.yaml");
    assert!(
        pubspec.contains("name: my_app\n"),
        "pubspec must name the package: {pubspec}"
    );
    assert!(
        pubspec.contains(&format!("flutter: '{FLUTTER_SDK_CONSTRAINT}'")),
        "pubspec must declare the Flutter SDK constraint: {pubspec}"
    );
    assert!(
        pubspec.contains(&format!("sdk: '{DART_SDK_CONSTRAINT}'")),
        "pubspec must declare the Dart SDK constraint: {pubspec}"
    );
    // A `sdk: flutter` dependency would require the SDK at install time; the
    // template must stay installable on an SDK-less machine.
    // (Dependency `sdk: flutter` sẽ đòi SDK lúc install; template phải cài được
    // trên máy không có SDK.)
    assert!(
        !pubspec.contains("sdk: flutter"),
        "pubspec must not depend on an SDK package: {pubspec}"
    );
    assert!(
        pubspec.contains("cupertino_icons"),
        "pubspec must declare a pub.dev dependency so install has work: {pubspec}"
    );

    let main = std::fs::read_to_string(dir.join("lib/main.dart")).expect("lib/main.dart");
    assert!(
        main.contains("MaterialApp("),
        "entry point must build a MaterialApp"
    );
    assert!(main.contains("runApp("), "entry point must call runApp");

    for file in [
        "analysis_options.yaml",
        ".gitignore",
        "README.md",
        "test/widget_test.dart",
    ] {
        assert!(dir.join(file).is_file(), "{file} missing");
    }

    // The adapter must recognise the project it just wrote; otherwise the
    // install lane would refuse it as an unknown app.
    // (Adapter phải nhận ra project nó vừa ghi; nếu không lane install sẽ từ
    // chối như app lạ.)
    assert_eq!(
        crate::detect_language(&dir),
        Some(crate::AppLanguage::Flutter),
        "the generated project must be detected as Flutter"
    );
    cleanup(&dir);
}

#[test]
fn project_name_is_normalised_into_a_dart_package_name() {
    // Dart rejects hyphens and capitals in a package name, so a scaffolded
    // `My-Cool-App` must still produce a manifest `pub get` accepts.
    // (Dart từ chối gạch nối và chữ hoa trong tên package, nên `My-Cool-App`
    // vẫn phải sinh manifest mà `pub get` chấp nhận.)
    assert_eq!(package_name("My-Cool-App"), "my_cool_app");
    assert_eq!(package_name("app2"), "app2");
    assert_eq!(package_name("2fast"), "app_2fast");
    // Non-ASCII letters are not valid Dart identifiers, so a run of them
    // collapses to one separator and the trailing dash still trims away.
    // (Chữ ngoài ASCII không hợp lệ trong identifier Dart, nên một chuỗi như
    // vậy gộp thành một dấu phân cách và gạch nối cuối vẫn bị trim.)
    assert_eq!(package_name("dự-án"), "d_n");
    assert_eq!(package_name("---"), "app");
}

#[test]
fn existing_destination_is_refused() {
    let dir = target("existing");
    std::fs::create_dir_all(&dir).expect("pre-create destination");
    let outcome = runtime().block_on(scaffold_project("my_app", &dir));
    assert!(outcome.is_err(), "an existing destination must fail closed");
    assert!(
        outcome.unwrap_err().to_string().contains("already exists"),
        "the refusal must say the directory exists"
    );
    cleanup(&dir);
}

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
}

/// A unique, not-yet-existing destination directory — thư mục đích mới.
fn target(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "mgc-app-scaffold-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ))
}

fn cleanup(dir: &Path) {
    std::fs::remove_dir_all(dir).ok();
}
