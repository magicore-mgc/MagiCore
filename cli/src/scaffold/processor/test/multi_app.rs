// Multi-platform application scaffold test.
// Kiểm thử scaffold ứng dụng đa nền tảng.

#[test]
fn test_multi_app_scaffold_writes_shared_and_all_platforms() {
    let root = tempfile::tempdir().unwrap();
    let project_dir = root.path().join("demo-multi");
    let config = ScaffoldConfig {
        core: "app".to_string(),
        sub_type: String::new(),
        frameworks: vec!["multi".to_string()],
        project_name: project_dir.to_string_lossy().to_string(),
        features: vec![],
        template_dir: PathBuf::new(),
    };

    let out = Scaffolder::scaffold(&config).unwrap();
    let proj_config = mgc_config::project::ProjectConfig::from_scaffold(
        Scaffolder::display_name(&out),
        "app",
        "",
        config.frameworks.clone(),
        "",
        config.features.clone(),
    );
    proj_config.save(&out).unwrap();
    for expected in [
        "mgc.toml",
        "shared/build.gradle.kts",
        "shared/src/commonMain/kotlin/demo_multi/Shared.kt",
        "android/build.gradle.kts",
        "android/app/build.gradle.kts",
        "android/settings.gradle.kts",
        "android/app/src/main/kotlin/Main.kt",
        "ios/Package.swift",
        "ios/Sources/demo-multi/main.swift",
        "ios/ObjcBridge/ObjcBridge.h",
        "ios/ObjcBridge/ObjcBridge.m",
        "react-native/package.json",
        "react-native/App.js",
        "flutter/pubspec.yaml",
        "flutter/lib/main.dart",
    ] {
        assert!(out.join(expected).exists(), "missing {expected}");
    }
    let shared_build = std::fs::read_to_string(out.join("shared/build.gradle.kts")).unwrap();
    assert!(
        shared_build.contains("baseName = \"demo-multi\""),
        "shared framework baseName"
    );
    let android_build = std::fs::read_to_string(out.join("android/app/build.gradle.kts")).unwrap();
    assert!(
        android_build.contains("implementation(project(\":shared\"))"),
        "android depends on shared"
    );
    let android_settings =
        std::fs::read_to_string(out.join("android/settings.gradle.kts")).unwrap();
    assert!(
        android_settings.contains("include(\":app\", \":shared\")"),
        "android includes shared"
    );
    let mgc = std::fs::read_to_string(out.join("mgc.toml")).unwrap();
    assert!(mgc.contains("ecosystem = \"app\""), "app ecosystem");
    assert!(mgc.contains("language = \"multi\""), "multi language");
    for platform in ["android", "ios", "react-native", "flutter"] {
        assert!(
            mgc.contains(&format!("\"{platform}\"")),
            "platform {platform}"
        );
    }
}
