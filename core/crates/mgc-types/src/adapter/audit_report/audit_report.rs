//! Audit aggregation result and the rule that only a completed clean scan is clean. — Kết quả tổng hợp audit và quy tắc chỉ scan hoàn tất, không finding mới là sạch.

use super::super::{ScannerStatus, Vulnerability};
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AuditReport {
    pub packages_audited: usize,
    pub vulnerability_count: usize,
    pub vulnerabilities: Vec<Vulnerability>,
    /// P0.6 FIX: Scanner availability status
    /// Indicates whether audit scanner was actually available and ran
    /// vs returning fake clean when scanner doesn't exist
    #[serde(default)]
    pub scanner_status: ScannerStatus,
}

impl AuditReport {
    pub fn clean(packages_audited: usize) -> Self {
        Self {
            packages_audited,
            vulnerability_count: 0,
            vulnerabilities: vec![],
            scanner_status: ScannerStatus::Available,
        }
    }

    /// Scanner executed successfully — full-coverage audit result.
    /// Scanner đã chạy thành công — kết quả audit phủ đầy đủ.
    pub fn scanned(packages_audited: usize, vulnerabilities: Vec<Vulnerability>) -> Self {
        Self {
            packages_audited,
            vulnerability_count: vulnerabilities.len(),
            vulnerabilities,
            scanner_status: ScannerStatus::Available,
        }
    }

    /// Tool missing on this machine — audit NOT performed (Tech Lead §1).
    /// Thiếu tool trên máy này — audit KHÔNG được thực hiện.
    pub fn tool_missing(tool: impl Into<String>, remediation: impl Into<String>) -> Self {
        Self {
            packages_audited: 0,
            vulnerability_count: 0,
            vulnerabilities: vec![],
            scanner_status: ScannerStatus::ToolMissing {
                tool: tool.into(),
                remediation: remediation.into(),
            },
        }
    }

    /// Dev-only: no scanner implemented for this ecosystem yet.
    /// Chỉ dùng khi phát triển: chưa có scanner cho ecosystem này.
    pub fn unsupported_ecosystem(ecosystem: impl Into<String>) -> Self {
        Self {
            packages_audited: 0,
            vulnerability_count: 0,
            vulnerabilities: vec![],
            scanner_status: ScannerStatus::UnsupportedEcosystem {
                ecosystem: ecosystem.into(),
            },
        }
    }

    /// Scanner ran but failed (schema drift, network, malformed output).
    /// Scanner chạy nhưng lỗi (đổi schema, mạng, output malformed).
    pub fn scanner_failed(scanner: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            packages_audited: 0,
            vulnerability_count: 0,
            vulnerabilities: vec![],
            scanner_status: ScannerStatus::Failed {
                scanner: scanner.into(),
                reason: reason.into(),
            },
        }
    }

    /// Clean ONLY when the scanner actually ran (Available) and found
    /// nothing. Any non-Available state is NOT clean — a public API caller
    /// must never read "clean" from an unverified report (Tech Lead
    /// P0-1 2026-09-09).
    /// Chỉ sạch khi scanner THẬT SỰ chạy (Available) và không tìm thấy gì.
    /// Mọi trạng thái khác KHÔNG sạch — caller ngoài không được đọc
    /// "sạch" từ report chưa xác thực.
    pub fn is_clean(&self) -> bool {
        matches!(self.scanner_status, ScannerStatus::Available)
            && self.vulnerabilities.is_empty()
            && self.vulnerability_count == 0
    }

    /// P0.6 FIX: Check if scanner actually ran
    pub fn scanner_available(&self) -> bool {
        matches!(self.scanner_status, ScannerStatus::Available)
    }
}
