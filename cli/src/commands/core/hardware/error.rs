//! Typed failures for Hardware package materialization.
//! Lỗi có kiểu khi materialize package Hardware.

use crate::scaffold::resolver::ScaffoldResolveError;
use std::error::Error as StdError;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub(super) enum HardwareMaterializeError {
    MissingLayer {
        framework: String,
    },
    LayerResolution {
        source: ScaffoldResolveError,
    },
    TargetNotDirectory {
        path: PathBuf,
    },
    TargetInspection {
        path: PathBuf,
        source: std::io::Error,
    },
    Scaffold {
        source: anyhow::Error,
    },
}

impl HardwareMaterializeError {
    pub(super) fn missing_layer(framework: &str) -> Self {
        Self::MissingLayer {
            framework: framework.to_owned(),
        }
    }

    pub(super) fn layer_resolution(source: ScaffoldResolveError) -> Self {
        Self::LayerResolution { source }
    }

    pub(super) fn target_not_directory(path: PathBuf) -> Self {
        Self::TargetNotDirectory { path }
    }

    pub(super) fn target_inspection(path: PathBuf, source: std::io::Error) -> Self {
        Self::TargetInspection { path, source }
    }

    pub(super) fn scaffold(source: anyhow::Error) -> Self {
        Self::Scaffold { source }
    }
}

impl fmt::Display for HardwareMaterializeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingLayer { framework } => write!(
                formatter,
                "Required hardware template layer 'hardware/{framework}' missing - no fallback available"
            ),
            Self::LayerResolution { source } => fmt::Display::fmt(source, formatter),
            Self::TargetNotDirectory { path } => write!(
                formatter,
                "Hardware package target '{}' exists and is not a real directory",
                path.display()
            ),
            Self::TargetInspection { path, source } => write!(
                formatter,
                "Cannot inspect Hardware package target '{}': {source}",
                path.display()
            ),
            Self::Scaffold { source } => fmt::Display::fmt(source, formatter),
        }
    }
}

impl StdError for HardwareMaterializeError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::LayerResolution { source } => Some(source),
            Self::TargetInspection { source, .. } => Some(source),
            Self::Scaffold { source } => Some(source.as_ref()),
            Self::MissingLayer { .. } | Self::TargetNotDirectory { .. } => None,
        }
    }
}
