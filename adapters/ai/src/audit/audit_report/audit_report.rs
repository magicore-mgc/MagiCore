//! Audit report state and severity counters.
//! Trạng thái báo cáo audit và bộ đếm severity.

use super::super::{Finding, Severity};

/// Results accumulated while scanning a model.
/// Kết quả được tích lũy trong lúc quét model.
#[derive(Debug, Clone)]
pub struct AuditReport {
    pub model_path: String,
    pub findings: Vec<Finding>,
    pub scanned_files: usize,
    pub passed: bool,
}

impl AuditReport {
    pub fn new(model_path: &str) -> Self {
        AuditReport {
            model_path: model_path.to_string(),
            findings: vec![],
            scanned_files: 0,
            passed: true,
        }
    }

    pub fn add_finding(&mut self, finding: Finding) {
        if finding.severity >= Severity::High {
            self.passed = false;
        }
        self.findings.push(finding);
    }

    pub fn critical_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|f| f.severity == Severity::Critical)
            .count()
    }

    pub fn high_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|f| f.severity == Severity::High)
            .count()
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
