//! Typed local errors for unsupported Python dependency syntax.
//! Lỗi cục bộ có kiểu cho cú pháp dependency Python chưa hỗ trợ.

use std::fmt;

use mgc_types::MgError;

#[derive(Debug)]
pub(super) enum PythonDependencyError {
    UnsupportedSyntax,
    InvalidVersionRequirement(String),
}

impl fmt::Display for PythonDependencyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSyntax => formatter.write_str(concat!(
                "PEP 508 extras, markers, URL references, and compound ",
                "constraints are unsupported by the native resolver"
            )),
            Self::InvalidVersionRequirement(requirement) => {
                write!(
                    formatter,
                    "unsupported PEP 508 version requirement in '{requirement}'"
                )
            }
        }
    }
}

impl std::error::Error for PythonDependencyError {}

impl From<PythonDependencyError> for MgError {
    fn from(error: PythonDependencyError) -> Self {
        Self::Anyhow(anyhow::Error::new(error))
    }
}
