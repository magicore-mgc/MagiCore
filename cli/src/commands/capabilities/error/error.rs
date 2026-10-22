//! Capability-local errors — lỗi riêng của capability.
//! Keeps the legacy diagnostic while retaining a typed cause.
//! Giữ thông báo cũ và cung cấp lỗi có kiểu.

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::capabilities) enum CapabilitiesError {
    #[error("Unknown core '{core}'")]
    UnknownCore { core: String },
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
