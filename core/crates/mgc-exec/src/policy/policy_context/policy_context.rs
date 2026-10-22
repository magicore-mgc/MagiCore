/// Lane execution context for exception lookup — ngữ cảnh lane để tra exception.
/// Framework/version come from project metadata; empty version matches only
/// manifests written with an empty version (fail-closed otherwise).
/// (Framework/version từ metadata project; version rỗng chỉ khớp manifest
/// ghi version rỗng (còn lại fail-closed).)
#[derive(Debug, Clone)]
pub struct PolicyContext {
    /// Framework name (e.g. `"next"`, or ecosystem fallback).
    /// Tên framework (vd `"next"`, hoặc fallback ecosystem).
    pub framework: String,
    /// Framework version, empty when unknown — version framework, rỗng khi chưa biết.
    pub version: String,
    /// Operating system (`std::env::consts::OS`) — hệ điều hành.
    pub os: String,
    /// Operation (`"dev"`, `"run"`, `"test"`, `"build"`, `"exec"`).
    /// Thao tác (`"dev"`, `"run"`, `"test"`, `"build"`, `"exec"`).
    pub operation: String,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
