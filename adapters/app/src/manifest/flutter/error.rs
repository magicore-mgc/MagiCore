//! Local Flutter manifest error taxonomy and MgError boundary mapping.
//! Taxonomy lỗi manifest Flutter cục bộ và mapping sang MgError.

use mgc_types::MgError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub(in crate::manifest) enum FlutterError {
    Shared(MgError),
    YamlParse {
        context: String,
        source: serde_yaml::Error,
    },
    YamlSerialize {
        source: serde_yaml::Error,
    },
    Other(String),
    Integrity(String),
    Unsupported {
        capability: &'static str,
        guidance: String,
    },
}

pub(in crate::manifest) type FlutterResult<T> = Result<T, FlutterError>;

impl fmt::Display for FlutterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Shared(error) => write!(formatter, "{error}"),
            Self::YamlParse { context, source } => write!(formatter, "{context}: {source}"),
            Self::YamlSerialize { source } => write!(formatter, "serialize pubspec.yaml: {source}"),
            Self::Other(message) => formatter.write_str(message),
            Self::Integrity(message) => {
                write!(formatter, "integrity verification failed: {message}")
            }
            Self::Unsupported {
                capability,
                guidance,
            } => {
                write!(
                    formatter,
                    "app core does not support '{capability}' yet: {guidance}"
                )
            }
        }
    }
}

impl Error for FlutterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Shared(error) => Some(error),
            Self::YamlParse { source, .. } | Self::YamlSerialize { source } => Some(source),
            Self::Other(_) | Self::Integrity(_) | Self::Unsupported { .. } => None,
        }
    }
}

impl From<MgError> for FlutterError {
    fn from(error: MgError) -> Self {
        Self::Shared(error)
    }
}

impl From<FlutterError> for MgError {
    fn from(error: FlutterError) -> Self {
        match error {
            FlutterError::Shared(error) => error,
            FlutterError::YamlParse { context, source } => {
                Self::Other(format!("{context}: {source}"))
            }
            FlutterError::YamlSerialize { source } => {
                Self::Other(format!("serialize pubspec.yaml: {source}"))
            }
            FlutterError::Other(message) => Self::Other(message),
            FlutterError::Integrity(message) => Self::Integrity(message),
            FlutterError::Unsupported {
                capability,
                guidance,
            } => Self::Unsupported {
                core: "app",
                capability,
                guidance,
            },
        }
    }
}
