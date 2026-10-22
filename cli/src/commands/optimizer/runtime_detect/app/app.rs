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

    // Check for Rust native (Cargo.toml with bin or no crate-type=lib) — kiểm tra ứng dụng Rust native
    if project_root.join("Cargo.toml").exists() {
        runtimes.push(DetectedRuntime::RustNative);
    }

    runtimes
}
