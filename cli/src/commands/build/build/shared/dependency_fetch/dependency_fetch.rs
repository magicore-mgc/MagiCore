//! Dependency-fetch mode for a native toolchain build.
//! Chế độ lấy dependency cho build bằng toolchain native.

/// How a build may obtain dependencies it does not already have.
///
/// `Online` (the default) appends no offline switch, so Cargo can fetch a
/// crate the managed store does not hold. `Offline` forces the offline switch
/// and keeps the fail-closed behavior: a missing crate fails the build with
/// Cargo's own message instead of a partial result.
/// (Chế độ lấy dependency. `Online` là mặc định — không thêm cờ offline để
/// Cargo tự tải crate chưa có; `Offline` ép offline và fail-closed.)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DependencyFetch {
    Online,
    Offline,
}

impl DependencyFetch {
    /// Map the `--offline` CLI flag to the mode the build passes to Cargo.
    /// (Ánh xạ cờ `--offline` sang chế độ dùng cho Cargo.)
    pub(crate) fn from_flag(offline: bool) -> Self {
        if offline { Self::Offline } else { Self::Online }
    }

    /// Whether a Cargo invocation must carry `--offline`.
    /// (Lời gọi Cargo có phải kèm `--offline` không.)
    pub(crate) fn is_offline(self) -> bool {
        matches!(self, Self::Offline)
    }
}
