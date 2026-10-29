//! Native/mobile application runtime recognition.
use super::super::{DetectedRuntime, project_text};
use std::path::Path;

pub(in crate::commands::optimizer) fn detect_app_runtime(
    project_root: &Path,
) -> Vec<DetectedRuntime> {
    let mut runtimes = vec![];

    // Check for Flutter (pubspec.yaml) — kiểm tra Flutter
    if project_root.join("pubspec.yaml").exists() {
        runtimes.push(DetectedRuntime::Flutter);
        return runtimes; // Flutter is exclusive for app — Flutter là độc quyền cho app
    }

    // Check for React Native (package.json + metro.config.js or react-native dependency) — kiểm tra React Native
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Some(content) = project_text(project_root, "package.json")
        && (content.contains("react-native") || project_root.join("metro.config.js").exists())
    {
        runtimes.push(DetectedRuntime::ReactNative);
        return runtimes;
    }

    // SwiftPM app: `mgc build`/`mgc dev` support Swift, so the optimizer must
    // recognise it too — otherwise a buildable app project reported
    // "No runtime detected … nothing was optimized".
    // (SwiftPM: `mgc build`/`mgc dev` hỗ trợ Swift nên optimizer cũng phải nhận
    // ra, nếu không project build được lại báo "không phát hiện runtime".)
    if project_root.join("Package.swift").is_file() {
        runtimes.push(DetectedRuntime::SwiftLib);
        return runtimes;
    }

    // Kotlin/Android app (Gradle Android plugin).
    // (App Kotlin/Android qua Gradle Android plugin.)
    for gradle in ["build.gradle", "build.gradle.kts"] {
        if let Some(content) = project_text(project_root, gradle)
            && (content.contains("com.android.application")
                || content.contains("org.jetbrains.kotlin.android"))
        {
            runtimes.push(DetectedRuntime::JvmLib);
            return runtimes;
        }
    }

    // Check for Rust native (Cargo.toml with bin or no crate-type=lib) — kiểm tra ứng dụng Rust native
    if project_root.join("Cargo.toml").exists() {
        runtimes.push(DetectedRuntime::RustNative);
    }

    runtimes
}
