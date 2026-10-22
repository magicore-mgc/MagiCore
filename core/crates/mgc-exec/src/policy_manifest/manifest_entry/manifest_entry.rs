use serde::{Deserialize, Serialize};

/// One exact read-only exception — một ngoại lệ chỉ đọc chính xác.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestEntry {
    /// Program basename, lowercase — basename chương trình, lowercase.
    pub program: String,
    /// Exact argument list — danh sách tham số chính xác.
    pub args_exact: Vec<String>,
    /// Framework the exception was verified for (e.g. `"next"`).
    /// Framework đã xác minh exception (vd `"next"`).
    pub framework: String,
    /// Framework version verified (e.g. `"16.3.8"`).
    /// Version framework đã xác minh (vd `"16.3.8"`).
    pub framework_version: String,
    /// Operating system (e.g. `"darwin"`) — hệ điều hành (vd `"darwin"`).
    pub os: String,
    /// Operation (e.g. `"dev"`) — thao tác (vd `"dev"`).
    pub operation: String,
    /// Why this invocation is needed — vì sao cần invocation này.
    pub reason: String,
    /// Expected observable output — output quan sát được kỳ vọng.
    pub expected: String,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
