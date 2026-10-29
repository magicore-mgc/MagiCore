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
    /// A completed scan with no findings.
    ///
    /// `packages_audited == 0` is NOT a completed scan: nothing was inspected, so
    /// reporting `Available` advertised "verified clean" for a project that was
    /// never looked at. `mgc audit` on a manifest-less project printed
    /// "✔ No vulnerabilities" with `scanner_status: "available"` and zero
    /// packages — the exact false assurance this type exists to prevent.
    ///
    /// The zero case keeps `packages_audited = 0` but reports `Partial` with the
    /// reason, so `is_clean()` is false and every consumer (SARIF driver name,
    /// exit code, CI gate) sees an unverified result.
    /// (`packages_audited == 0` KHÔNG phải scan hoàn tất: không có gì được kiểm
    /// nên báo `Available` là quảng cáo "đã xác minh sạch" cho project chưa từng
    /// được xem. `mgc audit` trên project không manifest từng in "✔ No
    /// vulnerabilities" với `available` và 0 package — đúng sự trấn an giả mà
    /// kiểu này sinh ra để chặn.)
    pub fn clean(packages_audited: usize) -> Self {
        if packages_audited == 0 {
            return Self {
                packages_audited: 0,
                vulnerability_count: 0,
                vulnerabilities: vec![],
                scanner_status: ScannerStatus::Partial {
                    scanned: 0,
                    skipped: 0,
                    reasons: vec![
                        "no packages were available to audit — run `mgc install` so the \
                         dependency graph exists, then audit again"
                            .to_string(),
                    ],
                },
            };
        }
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
    /// Whether this report may be printed as a verified clean result.
    ///
    /// Three conditions, and the third is the one that was missing: a scanner
    /// that completed (`Available`), no findings, AND at least one package
    /// actually inspected. `Available` with `packages_audited == 0` means the
    /// scanner had nothing to look at, so it verifies nothing.
    ///
    /// Guarding here rather than in `clean()` matters: six call sites build
    /// `AuditReport { scanner_status: Available, .. }` directly (terraform
    /// providers, OSV pins, OWASP parser, wasm provenance, …) and never pass
    /// through `clean()`. A per-constructor fix left those open; this predicate is
    /// the single place every consumer (`finish_and_print`, SARIF driver name,
    /// exit code, CI gate) already trusts.
    /// (Ba điều kiện, điều thứ ba từng thiếu: scanner hoàn tất (`Available`),
    /// không finding, VÀ ít nhất một package đã kiểm. Chặn ở đây thay vì trong
    /// `clean()` vì 6 nơi dựng `AuditReport { Available, .. }` trực tiếp, không
    /// qua `clean()`.)
    pub fn is_clean(&self) -> bool {
        matches!(self.scanner_status, ScannerStatus::Available)
            && self.vulnerabilities.is_empty()
            && self.vulnerability_count == 0
            && self.packages_audited > 0
    }

    /// P0.6 FIX: Check if scanner actually ran
    pub fn scanner_available(&self) -> bool {
        matches!(self.scanner_status, ScannerStatus::Available)
    }
}
