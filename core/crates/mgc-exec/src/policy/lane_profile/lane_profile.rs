use serde::{Deserialize, Serialize};

/// Active spawn-profile of a lane — profile hiệu lực của một lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneProfile {
    /// Framework name (e.g. `"next"`) — tên framework (vd `"next"`).
    pub framework: String,
    /// Locked framework version — version framework đã khóa.
    pub version: String,
    /// Operating system (e.g. `"darwin"`) — hệ điều hành (vd `"darwin"`).
    pub os: String,
    /// Operation (e.g. `"dev"`) — thao tác (vd `"dev"`).
    pub operation: String,
    /// Exact `"program args..."` strings observed as evidence.
    /// Chuỗi `"program args..."` chính xác đã quan sát làm bằng chứng.
    pub allowed_pm_invocations: Vec<String>,
    /// Package managers this lane observably requires at runtime
    /// (e.g. Angular needing `npm` at build). Non-empty means native mode
    /// refuses upfront: the lane needs `--compat-runner` or a native
    /// alternative (Spec rev4 §3.1 row 3).
    /// Package manager lane này chắc chắn cần lúc runtime (vd Angular cần
    /// `npm` lúc build). Không rỗng nghĩa là native mode từ chối upfront:
    /// lane cần `--compat-runner` hoặc đường native khác (Spec rev4 §3.1).
    #[serde(default)]
    pub requires_pm: Vec<String>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
