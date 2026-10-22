use super::super::super::super::*;
use crate::commands::build::implementation::run_allowlisted_tool;
use anyhow::Result;
use std::path::Path;

#[cfg(feature = "app")]
pub(crate) fn build_multi_app(root: &Path, v: &toml::Value) -> Result<()> {
    // Validate every configured target before invoking any toolchain.
    // Kiểm tra toàn bộ target trước khi gọi bất kỳ toolchain nào.
    let platforms = resolve_multi_platforms(v)?;

    let mut built = 0;
    let mut skipped = Vec::new();
    for platform in &platforms {
        let dir = root.join(platform);
        if !dir.exists() {
            skipped.push(format!("{platform} (directory missing)"));
            continue;
        }
        match platform.as_str() {
            "android" => {
                if tool_unavailable("gradle") {
                    skipped.push("android (gradle not found)".to_string());
                    continue;
                }
                run_allowlisted_tool(&dir, "gradle", &["build", "--offline"])?;
                built += 1;
            }
            "ios" => {
                if tool_unavailable("swift") {
                    skipped.push("ios (swift not found)".to_string());
                    continue;
                }
                run_allowlisted_tool(
                    &dir,
                    "swift",
                    &["build", "--skip-update", "--disable-automatic-resolution"],
                )?;
                built += 1;
            }
            "react-native" => {
                skipped.push("react-native (native runner unavailable)".to_string());
            }
            "flutter" => {
                if tool_unavailable("flutter") {
                    skipped.push("flutter (flutter toolchain not found)".to_string());
                    continue;
                }
                // `flutter build` without a target exits 2 ("Missing
                // target"). `web` is the universal desktop-CI target —
                // no Android SDK, no Xcode, works on all three runners
                // (P0 finding, 2026-09-12: the honest lifecycle matrix
                // caught the old bare `flutter build` failing).
                // `flutter build` thiếu target thì exit 2 ("Missing
                // target"). `web` là target phổ quát cho CI desktop —
                // không cần Android SDK, không cần Xcode, chạy được cả
                // ba runner (P0 finding 2026-09-12: matrix lifecycle
                // trung thực bắt được `flutter build` trần fail).
                run_allowlisted_tool(&dir, "flutter", &["build", "web", "--no-pub"])?;
                built += 1;
            }
            other => skipped.push(format!("{other} (unknown platform)")),
        }
    }
    finish_multi_build(built, &skipped)
}

#[cfg(feature = "app")]
pub(crate) fn resolve_multi_platforms(config: &toml::Value) -> Result<Vec<String>> {
    let Some(value) = config.get("app").and_then(|app| app.get("platforms")) else {
        return Ok(["android", "ios", "react-native", "flutter"]
            .into_iter()
            .map(String::from)
            .collect());
    };

    let Some(platforms) = value.as_array() else {
        return Err(anyhow::Error::new(
            super::error::AppMultiBuildError::InvalidPlatforms {
                reason: "expected an array of platform names".to_string(),
            },
        ));
    };
    if platforms.is_empty() {
        return Err(anyhow::Error::new(
            super::error::AppMultiBuildError::InvalidPlatforms {
                reason: "the explicit platform list must not be empty".to_string(),
            },
        ));
    }

    let mut selected = Vec::with_capacity(platforms.len());
    for (index, platform) in platforms.iter().enumerate() {
        let Some(platform) = platform.as_str() else {
            return Err(anyhow::Error::new(
                super::error::AppMultiBuildError::InvalidPlatforms {
                    reason: format!("entry {index} must be a string"),
                },
            ));
        };
        if !["android", "ios", "react-native", "flutter"].contains(&platform) {
            return Err(anyhow::Error::new(
                super::error::AppMultiBuildError::InvalidPlatforms {
                    reason: format!("unknown platform '{platform}' at entry {index}"),
                },
            ));
        }
        if selected.iter().any(|existing| existing == platform) {
            return Err(anyhow::Error::new(
                super::error::AppMultiBuildError::InvalidPlatforms {
                    reason: format!("duplicate platform '{platform}'"),
                },
            ));
        }
        selected.push(platform.to_string());
    }
    Ok(selected)
}

#[cfg(feature = "app")]
pub(crate) fn finish_multi_build(built: usize, skipped: &[String]) -> Result<()> {
    if !skipped.is_empty() {
        return Err(anyhow::Error::new(
            super::error::AppMultiBuildError::Incomplete {
                skipped: skipped.to_vec(),
            },
        ));
    }
    if built == 0 {
        return Err(crate::error::build_no_artifact());
    }
    Ok(())
}
