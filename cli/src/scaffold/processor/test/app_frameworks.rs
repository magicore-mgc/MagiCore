// Native application framework scaffold tests.
// Kiểm thử scaffold framework ứng dụng native.

#[test]
fn test_flutter_scaffold_has_testable_package_contract() {
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("app-demo");

    crate::scaffold::processors::app::AppProcessor::files(&target, "app-demo", "flutter").unwrap();

    let pubspec = std::fs::read_to_string(target.join("pubspec.yaml")).unwrap();
    assert!(
        pubspec.contains("environment:"),
        "Flutter SDK range missing"
    );
    assert!(
        pubspec.contains("sdk: flutter"),
        "Flutter SDK dependency missing"
    );
    assert!(
        pubspec.contains("flutter_test:"),
        "Flutter test dependency missing"
    );
    assert!(
        target.join("test/widget_test.dart").exists(),
        "Flutter lifecycle needs a real test target"
    );
    let web_index = std::fs::read_to_string(target.join("web/index.html")).unwrap();
    assert!(web_index.contains("$FLUTTER_BASE_HREF"));
}
#[test]
fn swift_scaffold_has_a_real_test_target() {
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("test-swift-app");

    crate::scaffold::processors::app::AppProcessor::files(&target, "test-swift-app", "swift")
        .unwrap();

    let manifest = std::fs::read_to_string(target.join("Package.swift")).unwrap();
    assert!(manifest.contains(".executableTarget(name: \"MagiCoreApp\")"));
    assert!(
        manifest
            .contains(".testTarget(name: \"MagiCoreAppTests\", dependencies: [\"MagiCoreApp\"])")
    );
    assert!(target.join("Sources/MagiCoreApp/App.swift").is_file());
    let tests =
        std::fs::read_to_string(target.join("Tests/MagiCoreAppTests/AppTests.swift")).unwrap();
    assert!(tests.contains("XCTAssertEqual(greeting(), \"MagiCore Swift app scaffold\")"));
}
#[test]
fn maui_scaffold_has_project_and_xaml_entrypoints() {
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("maui-demo");

    crate::scaffold::processors::app::AppProcessor::files(&target, "maui-demo", "maui").unwrap();

    let project = std::fs::read_to_string(target.join("maui-demo.csproj")).unwrap();
    assert!(project.contains("<UseMaui>true</UseMaui>"));
    assert!(target.join("MauiProgram.cs").is_file());
    assert!(target.join("App.xaml").is_file());
    assert!(target.join("App.xaml.cs").is_file());
    assert!(target.join("MainPage.xaml").is_file());
    assert!(target.join("MainPage.xaml.cs").is_file());
}
#[test]
fn tauri_scaffold_has_frontend_rust_and_v2_config() {
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("tauri-demo");

    crate::scaffold::processors::app::AppProcessor::files(&target, "tauri-demo", "tauri").unwrap();

    let package: serde_json::Value =
        serde_json::from_slice(&std::fs::read(target.join("package.json")).unwrap()).unwrap();
    let tauri: serde_json::Value =
        serde_json::from_slice(&std::fs::read(target.join("src-tauri/tauri.conf.json")).unwrap())
            .unwrap();
    assert_eq!(package["name"], "tauri-demo");
    assert!(target.join("src/main.js").is_file());
    assert!(target.join("dist/index.html").is_file());
    assert!(target.join("src-tauri/Cargo.toml").is_file());
    assert!(target.join("src-tauri/src/main.rs").is_file());
    assert_eq!(tauri["build"]["frontendDist"], "../dist");
}
#[test]
fn react_native_scaffold_has_expo_package_and_registered_entrypoint() {
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("native-demo");

    crate::scaffold::processors::app::AppProcessor::files(&target, "native-demo", "react-native")
        .unwrap();

    let package: serde_json::Value =
        serde_json::from_slice(&std::fs::read(target.join("package.json")).unwrap()).unwrap();
    let config: serde_json::Value =
        serde_json::from_slice(&std::fs::read(target.join("app.json")).unwrap()).unwrap();
    assert_eq!(package["name"], "native-demo");
    assert_eq!(package["dependencies"]["expo"], "~57.0.27");
    assert_eq!(package["dependencies"]["react-native"], "0.86.3");
    assert_eq!(package["scripts"]["start"], "expo start");
    assert_eq!(config["expo"]["slug"], "native-demo");
    assert!(target.join("index.js").is_file());
    assert!(target.join("App.js").is_file());
}
#[test]
fn unknown_app_framework_does_not_silently_generate_flutter() {
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("unknown-app");

    let result = crate::scaffold::processors::app::AppProcessor::files(
        &target,
        "unknown-app",
        "not-a-framework",
    );

    assert!(result.is_err());
    assert!(!target.join("pubspec.yaml").exists());
}
#[test]
fn flutter_scaffold_has_test_and_web_lifecycle_files() {
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("flutter-app");
    crate::scaffold::processors::app::AppProcessor::files(&target, "flutter-app", "flutter")
        .unwrap();

    let pubspec = std::fs::read_to_string(target.join("pubspec.yaml")).unwrap();
    assert!(pubspec.contains("environment:"));
    assert!(pubspec.contains("sdk: flutter"));
    assert!(
        pubspec.contains("flutter_test:"),
        "embedded Flutter scaffold must include flutter_test"
    );
    assert!(
        target.join("test/widget_test.dart").is_file(),
        "embedded Flutter scaffold must include a real test"
    );
    assert!(
        target.join("web/index.html").is_file(),
        "embedded Flutter scaffold must include web platform files"
    );
    assert!(
        target.join("web/manifest.json").is_file(),
        "embedded Flutter scaffold must include web manifest"
    );
    let web_index = std::fs::read_to_string(target.join("web/index.html")).unwrap();
    assert!(web_index.contains("$FLUTTER_BASE_HREF"));
}
