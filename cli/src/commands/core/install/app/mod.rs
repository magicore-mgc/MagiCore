// GATE-EXEMPT: typed error construction/rendering only; no dependency execution.
// Chỉ tạo/render lỗi typed; install execution vẫn qua owner implementation có dep_gate.
//! App dependency installation owner.
//! Owner install dependency App.

#[path = "error.rs"]
mod error;
#[path = "app.rs"]
mod implementation;

pub use implementation::install;
#[cfg(test)]
pub(crate) use implementation::install_flutter_native;

pub(crate) fn install_packages_error(packages: &[String]) -> anyhow::Error {
    anyhow::Error::new(error::AppInstallError::packages_require_add(packages))
}
