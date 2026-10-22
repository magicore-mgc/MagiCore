//! Select an explicit or latest release version.
//! Chọn phiên bản phát hành được yêu cầu hoặc tag mới nhất.

mod version;
pub(in crate::commands) use version::RELEASE_REPO;
#[cfg(test)]
pub(in crate::commands) use version::is_valid_version;
pub use version::resolve_version;
