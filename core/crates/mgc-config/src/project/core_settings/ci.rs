//! `mgc.toml` `[ci]` block: what CI is allowed to build and verify.
//! Khối `[ci]` trong `mgc.toml`: CI được phép build và kiểm gì.

use serde::{Deserialize, Serialize};

/// Declared CI commands. Empty values mean "not configured" — the CI
/// scaffold must refuse to generate a workflow instead of inventing steps.
/// Lệnh CI đã khai báo. Giá trị rỗng nghĩa là "chưa cấu hình" — scaffold CI
/// phải từ chối sinh workflow thay vì tự bịa bước.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CiConfig {
    /// Production build command (required for a verified workflow).
    /// Lệnh build production (bắt buộc cho workflow đã xác minh).
    #[serde(default)]
    pub build: String,
    /// Test command (required for a verified workflow).
    /// Lệnh test (bắt buộc cho workflow đã xác minh).
    #[serde(default)]
    pub test: String,
    /// Optional smoke/integration check run against the built artifact.
    /// Kiểm smoke/integration tuỳ chọn chạy trên artifact đã build.
    #[serde(default)]
    pub smoke: String,
    /// Optional Dockerfile path; enables production artifact verification.
    /// Đường dẫn Dockerfile tuỳ chọn; bật kiểm artifact production.
    #[serde(default)]
    pub production: String,
    /// Optional runtime image used to run the artifact in staging.
    /// Image runtime tuỳ chọn để chạy artifact trong staging.
    #[serde(default)]
    pub runtime_image: String,
}

impl CiConfig {
    /// A workflow is only verifiable when build and test are both declared.
    /// Workflow chỉ xác minh được khi cả build và test đều được khai báo.
    pub fn verifiable(&self) -> bool {
        !self.build.trim().is_empty() && !self.test.trim().is_empty()
    }

    /// Production artifact verification is enabled by a declared Dockerfile.
    /// Kiểm artifact production bật khi có Dockerfile được khai báo.
    pub fn verifies_production(&self) -> bool {
        !self.production.trim().is_empty()
    }
}
