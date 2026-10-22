use super::super::{TargetPlatform, find_ios_simulator};
use std::path::Path;

/// Command selected for an app build or device workflow.
/// Lệnh được chọn cho build app hoặc workflow thiết bị.
pub struct InstallCommand {
    pub tool: String,
    pub args: Vec<String>,
}

/// Sinh InstallCommand cho Flutter theo OS/platform.
///
/// macOS + Xcode → `flutter run -d <simulator_udid>`
/// Android       → `flutter run -d emulator` (hoặc attached device)
pub(in crate::commands::core::dev) fn flutter_dev_command(
    platform: &TargetPlatform,
    dry_run: bool,
) -> InstallCommand {
    match platform {
        TargetPlatform::IosSimulator => {
            // Tìm simulator có sẵn
            let device_arg = find_ios_simulator().unwrap_or_else(|| {
                if !dry_run {
                    mgc_ui::warning("No iOS simulator found — falling back to auto-detect device");
                }
                "auto".to_string()
            });
            if !dry_run {
                mgc_ui::info(&format!("Target: iOS Simulator ({})", device_arg));
            }
            InstallCommand {
                tool: "flutter".to_string(),
                args: if device_arg == "auto" {
                    vec!["run".to_string(), "--no-pub".to_string()]
                } else {
                    vec![
                        "run".to_string(),
                        "-d".to_string(),
                        device_arg,
                        "--no-pub".to_string(),
                    ]
                },
            }
        }
        TargetPlatform::Android => {
            if !dry_run {
                mgc_ui::info("Target: Android (emulator / attached device)");
            }
            InstallCommand {
                tool: "flutter".to_string(),
                args: vec![
                    "run".to_string(),
                    "-d".to_string(),
                    "android".to_string(),
                    "--no-pub".to_string(),
                ],
            }
        }
    }
}

/// Sinh InstallCommand cho Kotlin (Android-only).
///
/// Dùng `./gradlew installDebug` → `adb shell am start` nếu build OK.
/// Trên macOS vẫn chạy Android bình thường qua emulator.
pub(in crate::commands::core::dev) fn kotlin_dev_command(root: &Path) -> InstallCommand {
    // Kiểm tra có gradlew không (Gradle wrapper — chuẩn Android)
    let has_gradlew = root.join("gradlew").exists() || root.join("android/gradlew").exists();
    let gradle_bin = if has_gradlew { "./gradlew" } else { "gradle" };
    InstallCommand {
        tool: gradle_bin.to_string(),
        args: vec!["installDebug".to_string(), "--offline".to_string()],
    }
}

#[cfg(test)]
#[path = "test/command_builder.rs"]
mod tests;
