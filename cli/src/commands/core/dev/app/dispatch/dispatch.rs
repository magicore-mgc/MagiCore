//! Dispatch an app project to its existing language lane without widening support.
//! Chuyển dự án app tới lane ngôn ngữ hiện có, không mở rộng phạm vi hỗ trợ.

use super::super::{
    TargetPlatform, android_emulator_running, dev_ios, flutter_dev_command, kotlin_dev_command,
    read_app_id, run_android_device_command, run_tool_with_env,
};
use anyhow::Result;
use std::path::Path;

pub(in crate::commands::core::dev::app) async fn run_language(
    root: &Path,
    lang: mgc_app_adapter::AppLanguage,
    platform: TargetPlatform,
    dry_run: bool,
    env_opt: Option<Vec<(String, String)>>,
) -> Result<()> {
    match lang {
        mgc_app_adapter::AppLanguage::Flutter => {
            let cmd = flutter_dev_command(&platform, dry_run);
            if dry_run {
                mgc_ui::info(&format!(
                    "[dry-run] would run: {} {}",
                    cmd.tool,
                    cmd.args.join(" ")
                ));
                return Ok(());
            }
            mgc_ui::info(&format!("Running: {} {}", cmd.tool, cmd.args.join(" ")));
            run_tool_with_env(root, &cmd.tool, cmd.args.as_slice(), env_opt.clone())?;
            Ok(())
        }

        mgc_app_adapter::AppLanguage::Swift => {
            // Pre-flight freshness (Spec rev4 §8): Swift identities are
            // lowercase-normalized on both sides (registry `scope.name` and
            // git repo tails — see freshness projection docs).
            // (Freshness pre-flight (Spec rev4 §8): identity Swift chuẩn hóa
            // lowercase cả hai phía.)
            crate::commands::freshness::check_swift_freshness(root)?;
            dev_ios(root, true, dry_run, env_opt.clone()).await
        }

        mgc_app_adapter::AppLanguage::ObjC => dev_ios(root, false, dry_run, env_opt.clone()).await,

        mgc_app_adapter::AppLanguage::Kotlin => {
            // Kotlin = Android-only (không phân biệt OS host)
            if dry_run {
                mgc_ui::info("[dry-run] would run: ./gradlew installDebug");
                return Ok(());
            }
            let cmd = kotlin_dev_command(root);
            mgc_ui::info(&format!(
                "App dev (Kotlin/Android): {} {}",
                cmd.tool,
                cmd.args.join(" ")
            ));
            run_tool_with_env(root, &cmd.tool, cmd.args.as_slice(), env_opt.clone())?;
            // Sau installDebug: launch app qua adb nếu có emulator
            if android_emulator_running(root)? {
                // Đọc applicationId từ mgc.toml nếu có
                let app_id = read_app_id(root).unwrap_or_else(|| "com.example.app".to_string());
                mgc_ui::info(&format!(
                    "Launching app: adb shell am start -n {app_id}/.MainActivity"
                ));
                run_android_device_command(
                    root,
                    &[
                        "shell",
                        "am",
                        "start",
                        "-n",
                        &format!("{app_id}/.MainActivity"),
                    ],
                )?;
            }
            Ok(())
        }

        mgc_app_adapter::AppLanguage::ReactNative => {
            let _ = platform;
            let _ = dry_run;
            Err(crate::error::app_not_available(
                "react-native dev is blocked in beta until the MagiCore-native app runner is available",
            ))
        }

        mgc_app_adapter::AppLanguage::Multi => {
            // Multi: ưu tiên Flutter entry
            let flutter_dir = root.join("flutter");
            if !flutter_dir.exists() {
                return Err(crate::error::multi_dev_flutter_only());
            }
            let cmd = flutter_dev_command(&platform, dry_run);
            if dry_run {
                mgc_ui::info(&format!(
                    "[dry-run] would run: {} {} (in flutter/)",
                    cmd.tool,
                    cmd.args.join(" ")
                ));
                return Ok(());
            }
            mgc_ui::info(&format!(
                "App dev (Multi → flutter/): {} {}",
                cmd.tool,
                cmd.args.join(" ")
            ));
            run_tool_with_env(&flutter_dir, &cmd.tool, cmd.args.as_slice(), env_opt)?;
            Ok(())
        }
        // Returned before platform detection above; keep the dispatch
        // exhaustive and the unsupported message explicit.
        mgc_app_adapter::AppLanguage::Maui | mgc_app_adapter::AppLanguage::Tauri => {
            Err(crate::error::app_not_available(
                "MagiCore recognizes this app scaffold, but native dependency install and dev execution are not implemented for MAUI/Tauri yet",
            ))
        }
    }
}
