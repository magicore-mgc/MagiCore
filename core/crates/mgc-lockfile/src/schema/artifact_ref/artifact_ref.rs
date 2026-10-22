//! Defines the physical artifact reference stored beside a package pin.
//! Định nghĩa tham chiếu artifact vật lý được lưu cùng package pin.

use serde::{Deserialize, Serialize};

/// Physical artifact backing a pin: where the bytes live, how big, and how
/// they were verified — Tham chiếu artifact vật lý của pin: byte nằm đâu,
/// kích thước bao nhiêu, xác minh bằng cách nào.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArtifactRef {
    /// Artifact URL (registry tarball, git ref, …) — URL artifact (tarball registry, git ref, …).
    pub url: String,
    /// Artifact size in bytes (optional) — Kích thước artifact tính bằng byte (tùy chọn).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
    /// BLAKE3 content hash (hex) — Hash nội dung BLAKE3 (hex).
    pub content_hash: String,
    /// Which mirror/endpoint the bytes were downloaded from — Byte được tải về từ mirror/endpoint nào.
    pub downloaded_from: String,
    /// SRI integrity the materializer enforces (`sha256-…`/`sha512-…`) —
    /// None until a resolver/import lane supplies one. v4 install refuses
    /// pins without it instead of silently skipping verification.
    /// Integrity SRI mà materializer bắt buộc — None cho tới khi lane
    /// resolve/import cung cấp. Install v4 từ chối pin thiếu field này
    /// thay vì lặng lẽ bỏ qua verify.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integrity_sri: Option<String>,
}
