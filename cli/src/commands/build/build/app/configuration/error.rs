//! Typed failures owned by App build configuration.
//! Lỗi typed thuộc cấu hình build App.

use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub(super) enum AppConfigurationError {
    InvalidConfig {
        path: PathBuf,
        source: toml::de::Error,
    },
    ReadFailed {
        path: PathBuf,
        source: anyhow::Error,
    },
    InvalidLanguage {
        language: String,
    },
    AmbiguousLanguage {
        languages: Vec<String>,
    },
}

impl fmt::Display for AppConfigurationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfig { path, source } => {
                write!(
                    formatter,
                    "invalid App build config '{}': {source}",
                    path.display()
                )
            }
            Self::ReadFailed { path, source } => {
                write!(
                    formatter,
                    "cannot read App build config '{}': {source}",
                    path.display()
                )
            }
            Self::InvalidLanguage { language } => {
                write!(
                    formatter,
                    "unsupported or invalid App build language '{language}'"
                )
            }
            Self::AmbiguousLanguage { languages } => write!(
                formatter,
                "App build runtime is ambiguous ({}); set `app.language` explicitly",
                languages.join(", ")
            ),
        }
    }
}

impl std::error::Error for AppConfigurationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidConfig { source, .. } => Some(source),
            Self::ReadFailed { source, .. } => Some(source.as_ref()),
            Self::InvalidLanguage { .. } | Self::AmbiguousLanguage { .. } => None,
        }
    }
}
