use serde::Serialize;

/// Một dòng audit (JSON) — args nhận SẴN đã redact từ sanitizer.
#[derive(Debug, Clone, Serialize)]
pub struct AuditEntry {
    pub cmd: String,
    pub args: Vec<String>,
    pub cwd: String,
    pub exit_code: i32,
    pub duration_ms: u64,
    pub dry_run: bool,
    pub ts: u64,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
