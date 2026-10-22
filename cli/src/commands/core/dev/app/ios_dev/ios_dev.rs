use super::super::{
    boot_simulator_in, dev_scheme, find_ios_simulator_in, find_xcode_project, run_swift_package,
    run_tool_with_env, xcode_available_in,
};
use anyhow::{Result, bail};
use std::path::Path;

fn xcodebuild_args(scheme: String, simulator_udid: String, action: &str) -> Vec<String> {
    vec![
        "-scheme".to_string(),
        scheme,
        "-destination".to_string(),
        format!("id={simulator_udid}"),
        action.to_string(),
    ]
}

pub(in crate::commands::core::dev) async fn dev_ios(
    root: &Path,
    lang_is_swift: bool,
    dry_run: bool,
    env: Option<Vec<(String, String)>>,
) -> Result<()> {
    // T9: từ chối rõ ràng trên Linux/Windows
    if !cfg!(target_os = "macos") {
        bail!(
            "iOS Simulator only runs on macOS (host: {}). \
             Use `mgc dev app` on macOS or switch to Android target.",
            std::env::consts::OS
        );
    }
    if lang_is_swift && find_xcode_project(root).is_none() {
        return run_swift_package(root, dry_run, env);
    }
    if !xcode_available_in(Some(root)) {
        bail!(
            "Xcode command-line tools not found. Install with: xcode-select --install\n\
             Or set [app] dev_scheme in mgc.toml to use xcodebuild."
        );
    }

    // ObjC: dùng scheme từ mgc.toml
    if !lang_is_swift {
        let Some(scheme) = dev_scheme(root) else {
            let Some(proj) = find_xcode_project(root) else {
                return Err(crate::error::xcode_project_missing_short());
            };
            return Err(crate::error::objc_dev_needs_xcode(&proj));
        };
        let simulator_udid =
            find_ios_simulator_in(Some(root)).unwrap_or_else(|| "iPhone 16".to_string());
        if !dry_run {
            boot_simulator_in(&simulator_udid, Some(root)).ok();
        }
        let args = xcodebuild_args(scheme, simulator_udid, "build");
        if dry_run {
            mgc_ui::info(&format!(
                "[dry-run] would run: xcodebuild {}",
                args.join(" ")
            ));
            return Ok(());
        }
        mgc_ui::info(&format!("App dev (ObjC): xcodebuild {}", args.join(" ")));
        return run_tool_with_env(root, "xcodebuild", &args, env);
    }

    // Swift: `swift run` (macOS CLI) hoặc xcodebuild nếu có .xcodeproj
    let has_xcodeproj = find_xcode_project(root).is_some();
    if has_xcodeproj {
        let scheme = dev_scheme(root).unwrap_or_else(|| {
            root.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "App".to_string())
        });
        let simulator_udid =
            find_ios_simulator_in(Some(root)).unwrap_or_else(|| "iPhone 16".to_string());
        if !dry_run {
            boot_simulator_in(&simulator_udid, Some(root)).ok();
        }
        let args = xcodebuild_args(scheme, simulator_udid, "run");
        if dry_run {
            mgc_ui::info(&format!(
                "[dry-run] would run: xcodebuild {}",
                args.join(" ")
            ));
            return Ok(());
        }
        mgc_ui::info(&format!(
            "App dev (Swift/Xcode): xcodebuild {}",
            args.join(" ")
        ));
        return run_tool_with_env(root, "xcodebuild", &args, env);
    }

    // Project layout can change after the Xcode probe; keep the SwiftPM path
    // explicit if the Xcode project disappeared between the two checks.
    // (Cấu trúc có thể đổi sau khi probe Xcode; vẫn giữ nhánh SwiftPM rõ ràng.)
    run_swift_package(root, dry_run, env)
}

#[cfg(test)]
#[path = "test/ios_dev.rs"]
mod tests;
