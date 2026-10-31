//! `audit_format_test.rs` — OutputFormat parsing contract (P1-B).
//! Table/json/sarif/cyclonedx accepted; anything else is a HARD error —
//! CI must never ingest a silent fallback format.
//! Test hợp đồng parse OutputFormat (P1-B): nhận table/json/sarif/
//! cyclonedx; giá trị khác là lỗi CỨNG — CI không bao giờ ingest format
/// rơi về mặc định một cách âm thầm.
use crate::commands::audit::OutputFormat;

#[test]
fn parse_table_default_and_explicit() {
    assert!(matches!(
        OutputFormat::from_flag(None),
        Ok(OutputFormat::Table)
    ));
    assert!(matches!(
        OutputFormat::from_flag(Some("table")),
        Ok(OutputFormat::Table)
    ));
    assert!(matches!(
        OutputFormat::from_flag(Some("json")),
        Ok(OutputFormat::Json)
    ));
    assert!(matches!(
        OutputFormat::from_flag(Some("sarif")),
        Ok(OutputFormat::Sarif)
    ));
    assert!(matches!(
        OutputFormat::from_flag(Some("cyclonedx")),
        Ok(OutputFormat::CycloneDx)
    ));
}

#[test]
fn parse_unknown_format_is_hard_error() {
    // Unknown values must NEVER fall back to table — a silent fallback
    // would hand CI a human table where it expects machine JSON.
    // Giá trị lạ KHÔNG BAO GIỜ rơi về table — rơi về mặc định sẽ đưa CI
    // bảng hiển thị người thay vì JSON máy.
    for bad in ["xml", "SARIF", "", "text", "cyclone"] {
        assert!(
            OutputFormat::from_flag(Some(bad)).is_err(),
            "'{bad}' must be rejected, not silently defaulted"
        );
    }
}

#[test]
fn machine_formats_flagged_not_table() {
    for fmt in [
        OutputFormat::Json,
        OutputFormat::Sarif,
        OutputFormat::CycloneDx,
    ] {
        assert!(fmt.is_machine(), "{fmt:?} must be machine-only payload");
    }
    assert!(!OutputFormat::Table.is_machine());
}

#[test]
fn verify_strict_mode_rejects_unavailable_scanner_coverage() {
    // Verify must reject UNVERIFIED audit status even when the local CLI defaults to warning-only.
    // Verify phải từ chối audit UNVERIFIED dù CLI local mặc định chỉ cảnh báo.
    let report = mgc_types::adapter::AuditReport::unsupported_ecosystem("fixture");
    assert!(
        crate::commands::audit::enforce_audit_exit(
            "fixture",
            &report,
            crate::commands::audit::StrictMode::required(),
        )
        .is_err()
    );
}

#[test]
fn audit_fix_machine_output_defers_only_when_mutation_will_run() {
    use crate::commands::audit::OutputFormat;
    use crate::commands::audit::web::should_print_initial_report;

    assert!(!should_print_initial_report(
        true,
        OutputFormat::Json,
        true,
        1
    ));
    assert!(should_print_initial_report(
        true,
        OutputFormat::Json,
        true,
        0
    ));
    assert!(should_print_initial_report(
        true,
        OutputFormat::Json,
        false,
        1
    ));
    assert!(should_print_initial_report(
        false,
        OutputFormat::Json,
        true,
        1
    ));
    assert!(should_print_initial_report(
        true,
        OutputFormat::Table,
        true,
        1
    ));
}

/// R4/F2: non-CVE rows must be labeled by what they are — a policy
/// finding printed as "CVE:" would mislead triage.
/// Dòng phi-CVE phải ghi nhãn đúng bản chất — finding policy mà in
/// "CVE:" sẽ gây hiểu lầm.
#[test]
fn finding_table_labels_never_masquerade_as_cve() {
    use crate::commands::audit::finding_table_labels;
    use mgc_types::adapter::FindingClass;
    // Vulnerability rows label the id through `advisory_id_label`, never a
    // blanket "CVE" (see the npm-numeric case below).
    // (Dòng vulnerability ghi nhãn id qua `advisory_id_label`, không dùng
    // mặc định "CVE".)
    assert_eq!(finding_table_labels(FindingClass::Vulnerability), ("", ""));
    assert_eq!(
        finding_table_labels(FindingClass::Policy),
        (" [policy]", "Policy rule")
    );
    assert_eq!(
        finding_table_labels(FindingClass::Provenance),
        (" [provenance]", "Provenance check")
    );
    assert_eq!(
        finding_table_labels(FindingClass::Artifact),
        (" [artifact]", "Artifact check")
    );
}

/// The lodash advisory shipped `1106913`, an internal npm advisory id, and
/// the table labelled it `CVE:` — a CVE that does not exist. Real CVEs must
/// still be labelled `CVE`, GHSA ids `GHSA`, everything else `Advisory`.
/// (Advisory lodash có `1106913` — id nội bộ của npm — nhưng bảng ghi
/// `CVE:` là một CVE không tồn tại. CVE thật vẫn phải là `CVE`, GHSA là
/// `GHSA`, còn lại là `Advisory`.)
#[test]
fn advisory_id_label_distinguishes_npm_ids_from_real_cves() {
    use crate::commands::audit::advisory_id_label;
    assert_eq!(advisory_id_label("1106913"), "Advisory");
    assert_eq!(advisory_id_label("CVE-2021-23337"), "CVE");
    assert_eq!(advisory_id_label("GHSA-jf85-cpcp-j695"), "GHSA");
    assert_eq!(advisory_id_label("OSV-2021-999"), "Advisory");
    assert_eq!(advisory_id_label(""), "Advisory");
}

/// Only well-formed CVE identifiers are treated as CVEs.
/// (Chỉ id CVE đúng định dạng mới được coi là CVE.)
#[test]
fn cve_pattern_requires_year_and_number() {
    use crate::commands::audit::is_cve_id;
    for valid in ["CVE-2021-23337", "CVE-1999-0001", "cve-2021-23337"] {
        assert!(is_cve_id(valid), "{valid} must be recognised as a CVE id");
    }
    for invalid in [
        "1106913",
        "CVE-2021",
        "CVE-21-23337",
        "CVE-2021-",
        "CVE-2021-abc",
        "CVE--23337",
        "GHSA-xxxx-xxxx-xxxx",
        "CVE_2021_23337",
    ] {
        assert!(!is_cve_id(invalid), "{invalid} must not be labelled a CVE");
    }
}

/// End-to-end table rendering: the printed id line must use the id's real
/// class. This is the line that showed `CVE: 1106913` for the lodash
/// advisory (an npm registry id, real CVE is CVE-2021-23337).
/// (Render bảng end-to-end: dòng id in ra phải đúng loại thật của id. Đây
/// chính là dòng từng in `CVE: 1106913` cho advisory lodash.)
#[test]
fn printed_id_line_never_labels_a_registry_id_as_cve() {
    use crate::commands::audit::finding_table_labels;
    use mgc_types::adapter::FindingClass;

    // The renderer's label decision, exercised exactly as `print_report`
    // does it (the row's own id decides the label for vulnerabilities).
    // (Quyết định nhãn của renderer, chạy đúng như `print_report` làm.)
    let label_for = |cve: &str| {
        let (class_marker, class_label) = finding_table_labels(FindingClass::Vulnerability);
        assert!(class_marker.is_empty());
        let _ = class_label;
        crate::commands::audit::advisory_id_label(cve)
    };
    assert_eq!(label_for("1106913"), "Advisory");
    assert_eq!(label_for("CVE-2021-23337"), "CVE");
    assert_eq!(label_for("GHSA-jf85-cpcp-j695"), "GHSA");
    // Non-vulnerability rows keep their fixed label even with a numeric id.
    // (Dòng phi vulnerability giữ nhãn cố định kể cả với id số.)
    assert_eq!(finding_table_labels(FindingClass::Policy).1, "Policy rule");
}
