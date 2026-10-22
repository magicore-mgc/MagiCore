use crate::commands::build::implementation::project_manifest_present;
use anyhow::Result;
use std::path::Path;

/// Resolve an App build target without guessing a default runtime. An invalid
/// project config or an unknown explicit language must not silently route a
/// project into Flutter's external build toolchain.
/// Không đoán runtime mặc định; config hỏng/không biết ngôn ngữ không được
/// âm thầm chạy Flutter.
#[cfg(all(test, feature = "app"))]
pub(crate) fn resolve_app_language(root: &Path) -> Result<&'static str> {
    let config = read_app_build_config(root)?;
    resolve_app_language_from_config(root, config.as_ref())
}

#[cfg(feature = "app")]
pub(in crate::commands::build::implementation) fn read_app_build_config(
    root: &Path,
) -> Result<Option<toml::Value>> {
    let path = root.join("mgc.toml");
    match mgc_config::project::read_regular_project_text(&path, "project config") {
        Ok(Some(contents)) => Ok(Some(toml::from_str::<toml::Value>(&contents).map_err(
            |source| {
                anyhow::Error::new(super::error::AppConfigurationError::InvalidConfig {
                    path: path.clone(),
                    source,
                })
            },
        )?)),
        Ok(None) => Ok(None),
        Err(source) => Err(anyhow::Error::new(
            super::error::AppConfigurationError::ReadFailed {
                path: path.clone(),
                source,
            },
        )),
    }
}

#[cfg(feature = "app")]
pub(in crate::commands::build::implementation) fn resolve_app_language_from_config(
    root: &Path,
    config: Option<&toml::Value>,
) -> Result<&'static str> {
    if let Some(language) = config
        .and_then(|value| value.get("app"))
        .and_then(|app| app.get("language"))
    {
        let language = language.as_str().ok_or_else(|| {
            anyhow::Error::new(super::error::AppConfigurationError::InvalidLanguage {
                language: "<non-string>".to_string(),
            })
        })?;
        return match language {
            "flutter" => Ok("flutter"),
            "kotlin" => Ok("kotlin"),
            "swift" => Ok("swift"),
            "multi" => Ok("multi"),
            other => Err(anyhow::Error::new(
                super::error::AppConfigurationError::InvalidLanguage {
                    language: other.to_string(),
                },
            )),
        };
    }
    infer_app_language(root)?
        .ok_or_else(|| crate::error::no_framework_detected("app language", root))
}

#[cfg(feature = "app")]
pub(in crate::commands::build::implementation) fn infer_app_language(
    root: &Path,
) -> Result<Option<&'static str>> {
    let candidates = [
        ("Package.swift", "swift"),
        ("settings.gradle", "kotlin"),
        ("settings.gradle.kts", "kotlin"),
        ("build.gradle", "kotlin"),
        ("build.gradle.kts", "kotlin"),
        ("pubspec.yaml", "flutter"),
    ];
    let mut found = Vec::new();
    for (marker, language) in candidates {
        if project_manifest_present(root, marker)? && !found.contains(&language) {
            found.push(language);
        }
    }
    match found.as_slice() {
        [] => Ok(None),
        [language] => Ok(Some(*language)),
        _ => Err(anyhow::Error::new(
            super::error::AppConfigurationError::AmbiguousLanguage {
                languages: found
                    .iter()
                    .map(|language| (*language).to_string())
                    .collect(),
            },
        )),
    }
}
