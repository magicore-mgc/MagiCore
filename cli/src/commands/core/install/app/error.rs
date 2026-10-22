// GATE-EXEMPT: typed error construction/rendering only; no dependency execution.
// Chỉ tạo/render lỗi typed; install execution vẫn qua owner implementation có dep_gate.
//! Typed errors owned by App dependency installation.
//! Lỗi có kiểu thuộc thao tác cài dependency App.

use std::error::Error as StdError;
use std::fmt;

/// Install-specific refusal for package arguments passed to `install-app`.
/// Lỗi install riêng cho package arguments truyền vào `install-app`.
#[derive(Debug)]
pub(super) enum AppInstallError {
    PackagesRequireAdd { packages: Vec<String> },
}

impl AppInstallError {
    pub(super) fn packages_require_add(packages: &[String]) -> Self {
        Self::PackagesRequireAdd {
            packages: packages.to_vec(),
        }
    }
}

impl fmt::Display for AppInstallError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PackagesRequireAdd { packages } => write!(
                formatter,
                "`install-app` takes no packages (got {packages:?}) — install replays the existing graph/lock through the native pipeline; add packages with `mgc add-app <package>`"
            ),
        }
    }
}

impl StdError for AppInstallError {}
