//! `mgc dev app` coordinator. Platform and language execution live in their owner modules.
//! Bộ điều phối `mgc dev app`; platform và chạy từng ngôn ngữ nằm ở owner riêng.

use super::{TargetPlatform, detect_app_runtime, detect_target_platform, find_xcode_project};
use crate::commands::core::app::{language, project_root};
use anyhow::Result;

/// `mgc dev app` — T9: chọn platform dựa theo OS host.
///
/// macOS + Xcode → iOS Simulator (tự tìm UDID booted/available)
/// Linux / Windows → Android Emulator (flutter -d android / gradlew installDebug)
pub async fn dev(dry_run: bool) -> Result<()> {
    let root = project_root()?;
    let lang = language(&root)?;
    if matches!(
        lang,
        mgc_app_adapter::AppLanguage::Maui | mgc_app_adapter::AppLanguage::Tauri
    ) {
        return Err(crate::error::app_not_available(
            "MagiCore recognizes this app scaffold, but native dependency install and dev execution are not implemented for MAUI/Tauri yet",
        ));
    }
    // Pre-flight freshness applies only to Flutter's pubspec/lock pair.
    // (Freshness pre-flight chỉ áp dụng cho cặp pubspec/lock của Flutter.)
    if lang == mgc_app_adapter::AppLanguage::Flutter {
        crate::commands::freshness::check_flutter_freshness(&root)?;
    }

    let platform = detect_target_platform();
    let target_label =
        if lang == mgc_app_adapter::AppLanguage::Swift && find_xcode_project(&root).is_none() {
            "SwiftPM CLI"
        } else {
            match &platform {
                TargetPlatform::IosSimulator => "iOS Simulator",
                TargetPlatform::Android => "Android",
            }
        };

    if dry_run {
        mgc_ui::info(&format!(
            "[dry-run] mgc dev app — OS: {}, target: {}, lang: {:?}",
            std::env::consts::OS,
            target_label,
            lang
        ));
    } else {
        mgc_ui::info(&format!(
            "mgc dev app — OS: {}, target: {}",
            std::env::consts::OS,
            target_label
        ));
    }

    // Load optimizer env for app runtime
    // Tải env optimizer cho runtime app
    let runtime = detect_app_runtime(&root, &lang);
    let optimizer_envs =
        crate::commands::optimizer::env_loader::load_optimizer_env(&root, &runtime)?;
    let env_vec: Vec<(String, String)> = optimizer_envs.into_iter().collect();
    let env_opt = if env_vec.is_empty() {
        None
    } else {
        Some(env_vec)
    };

    super::dispatch::run_language(&root, lang, platform, dry_run, env_opt).await
}
