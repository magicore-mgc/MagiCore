//! Defines the serialized environment report model.
//! Khai báo model report môi trường được serialize.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DoctorReport {
    pub toolchain: Vec<Tool>,
    pub store_dir: String,
    pub store_writable: bool,
    pub disk_free_bytes: u64,
    pub registries_reachable: Vec<RegistryStatus>,
    pub health_status: &'static str,
    pub detected_issues: Vec<DiagnosticIssue>,
    pub suggested_actions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DiagnosticIssue {
    pub code: &'static str,
    /// Severity is a stable machine-readable level: CRITICAL, WARNING or INFO.
    /// Severity là mức ổn định để máy đọc: CRITICAL, WARNING hoặc INFO.
    pub severity: &'static str,
    pub message: String,
    pub fix_command: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Tool {
    pub name: &'static str,
    pub path: Option<String>,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct RegistryStatus {
    pub host: String,
    pub reachable: bool,
}
