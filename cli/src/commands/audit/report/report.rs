//! Render human tables and machine-readable audit payloads.
//! Render bảng human và payload audit cho máy.
use super::*;
use anyhow::Result;
use colored::Colorize;
use mgc_types::adapter::AuditReport;

#[path = "machine_payload/mod.rs"]
mod machine_payload;

use machine_payload::print_machine_payload;

/// Render an audit report with zero exit decisions (always Ok unless
/// rendering itself fails). Used by --fix flows before mutating.
/// (Chỉ in report — không quyết exit.)
pub(crate) fn print_audit_report(
    core: &str,
    report: &AuditReport,
    fmt: OutputFormat,
) -> Result<()> {
    // Machine formats: emit the payload and keep the SAME exit contract —
    // the human table is skipped entirely, CI parses structured data.
    // Format máy: phát payload và giữ NGUYÊN exit contract — bỏ bảng
    // hiển thị, CI parse dữ liệu có cấu trúc.
    if fmt.is_machine() {
        print_machine_payload(report, fmt)?;
    } else {
        print_table_report(core, report)?;
    }
    Ok(())
}

/// Table-path rendering (no exit decisions).
/// (In bảng — không quyết exit.)
fn print_table_report(core: &str, report: &AuditReport) -> Result<()> {
    if !mgc_ui::is_quiet() {
        mgc_ui::blank_line();
        println!(
            "🛡️  {}",
            format!("MagiCore Security Audit ({core} core)")
                .bold()
                .cyan()
        );
    }

    // Fail-closed classification over the five-state contract (Tech Lead
    // §1). Only `Available` may print "clean"; every other state is
    // UNVERIFIED — the banner below always prints locally; strictness
    // only moves the EXIT into enforce_audit_exit.
    // Phân loại fail-closed trên contract 5 trạng thái. Chỉ `Available` mới
    // được in "clean"; mọi trạng thái khác là UNVERIFIED — banner luôn in,
    // strict chỉ đổi exit ở enforce.
    if !report.scanner_available() {
        use mgc_types::adapter::ScannerStatus;

        let (headline, detail) = match &report.scanner_status {
            ScannerStatus::Partial {
                scanned,
                skipped,
                reasons,
            } => (
                format!(
                    "{core} audit PARTIAL: {scanned} scanned, {skipped} skipped (strict CI must fail)"
                ),
                format!("  Skipped reasons: {}", reasons.join("; ")),
            ),
            ScannerStatus::ToolMissing { tool, remediation } => (
                format!("{core} audit NOT performed — required tool missing: {tool}"),
                format!("  Remediation: {remediation}"),
            ),
            ScannerStatus::UnsupportedEcosystem { ecosystem } => (
                format!(
                    "{core} audit NOT performed — no scanner implemented for ecosystem '{ecosystem}' (dev-only state)"
                ),
                "  This core cannot be claimed complete while this state is reachable.".to_string(),
            ),
            ScannerStatus::Failed { scanner, reason } => (
                format!("{core} audit scanner FAILED: {scanner}"),
                format!("  Reason: {reason}"),
            ),
            ScannerStatus::Available => {
                return Err(crate::error::audit_unknown_scanner_state(core));
            }
        };
        eprintln!("WARN: {headline}");
        eprintln!("{detail}");
        // A Partial report can still carry REAL findings (OSV fallback
        // lanes scan direct pins while skipping the rest) — show what we
        // know BEFORE the UNVERIFIED banner, never hide findings behind
        // an incomplete status.
        // Report Partial vẫn có thể mang finding THẬT — in những gì đã
        // biết TRƯỚC banner UNVERIFIED, không giấu finding sau trạng thái
        // chưa hoàn tất.
        if !report.vulnerabilities.is_empty() {
            print_report(report);
        }
        eprintln!("  Audit NOT performed/complete — this run is UNVERIFIED, not clean.");
        eprintln!("  Status: UNVERIFIED (exit decided by enforce_audit_exit)");
        eprintln!(
            "  Set MGC_AUDIT_STRICT=1 (or pass --audit-strict) to fail on unavailable scanner in CI."
        );
        return Ok(());
    }

    print_report(report);

    if report.vulnerability_count == 0 {
        mgc_ui::success("No vulnerabilities reported by the configured provider");
    }
    Ok(())
}

/// Human-table label for a finding's identifier (R4/F2): policy,
/// provenance, and artifact rows must never masquerade as CVEs — the id
/// line is labeled by what the row actually is.
/// (Nhãn bảng theo class: dòng phi-CVE không được giả dạng CVE.)
pub(crate) fn finding_table_labels(
    class: mgc_types::adapter::FindingClass,
) -> (&'static str, &'static str) {
    use mgc_types::adapter::FindingClass;
    match class {
        // Vulnerability rows carry their own id label (see `id_label`).
        // (Dòng vulnerability tự ghi nhãn id của nó.)
        FindingClass::Vulnerability => ("", ""),
        FindingClass::Policy => (" [policy]", "Policy rule"),
        FindingClass::Provenance => (" [provenance]", "Provenance check"),
        FindingClass::Artifact => (" [artifact]", "Artifact check"),
    }
}

/// Label an advisory identifier by what it actually IS.
///
/// npm's bulk advisory `id` is a registry number (e.g. `1106913`), GitHub
/// advisories are `GHSA-xxxx`, and only `CVE-<year>-<number>` ids are
/// CVEs. Mislabelling an internal id as `CVE:` sends triage to a CVE that
/// does not exist.
/// (Gán nhãn id advisory theo đúng bản chất. `id` của npm bulk advisory là
/// số registry, GHSA là `GHSA-xxxx`, chỉ id `CVE-<năm>-<số>` mới là CVE.
/// Gắn nhãn `CVE:` cho id nội bộ sẽ đẩy người dò sang một CVE không tồn tại.)
pub(crate) fn advisory_id_label(id: &str) -> &'static str {
    if is_cve_id(id) {
        "CVE"
    } else if id.starts_with("GHSA-") {
        "GHSA"
    } else {
        "Advisory"
    }
}

/// True only for a well-formed `CVE-<4 digits>-<number>` identifier. The
/// prefix match is case-insensitive because feeds vary, and the year must
/// be exactly four digits.
/// (Đúng chỉ với id `CVE-<4 chữ số>-<số>` hợp lệ. Prefix so khớp không
/// phân biệt hoa thường vì nguồn feed khác nhau, và năm phải đủ 4 chữ số.)
pub(crate) fn is_cve_id(id: &str) -> bool {
    let Some((prefix, rest)) = id.split_at_checked(4) else {
        return false;
    };
    if !prefix.eq_ignore_ascii_case("CVE-") {
        return false;
    }
    let Some((year, number)) = rest.split_once('-') else {
        return false;
    };
    year.len() == 4
        && year.bytes().all(|byte| byte.is_ascii_digit())
        && !number.is_empty()
        && number.bytes().all(|byte| byte.is_ascii_digit())
}

/// Render the typed report — shared across cores so every ecosystem gets
/// the same table: package counts, per-finding severity, advisory links.
/// Render report typed — dùng chung mọi core: mọi ecosystem cùng một bảng
/// kết quả: số package, severity từng finding, link advisory.
pub(crate) fn print_report(report: &AuditReport) {
    use colored::*;
    use mgc_types::adapter::VulnerabilitySeverity;

    mgc_ui::blank_line();
    println!("{}", "Audit Report".bold().underline());
    println!("  Packages audited: {}", report.packages_audited);
    println!("  Vulnerabilities: {}", report.vulnerability_count);

    for vuln in &report.vulnerabilities {
        let severity = match vuln.severity_level {
            VulnerabilitySeverity::Critical | VulnerabilitySeverity::High => {
                vuln.severity.red().bold()
            }
            VulnerabilitySeverity::Medium => vuln.severity.yellow().bold(),
            _ => vuln.severity.normal(),
        };
        mgc_ui::blank_line();
        let (class_marker, class_label) = finding_table_labels(vuln.finding_class);
        println!(
            "{}{} {} in {}",
            severity,
            class_marker,
            vuln.title.bold(),
            vuln.package
        );
        if !vuln.cve.is_empty() {
            // Vulnerability rows take the id label from the id itself; other
            // classes keep their fixed label.
            // (Dòng vulnerability lấy nhãn id từ chính id; class khác giữ
            // nhãn cố định.)
            let id_label = match vuln.finding_class {
                mgc_types::adapter::FindingClass::Vulnerability => advisory_id_label(&vuln.cve),
                _ => class_label,
            };
            println!("  {id_label}: {}", vuln.cve);
        }
        if let Some(patched) = &vuln.patched_versions {
            println!("  Patched versions: {}", patched);
        }
        if let Some(url) = &vuln.url {
            println!("  Advisory: {}", url);
        }
    }

    if report.vulnerability_count > 0 {
        mgc_ui::warning("Audit failed. Review the advisories above before shipping.");
    }
}
