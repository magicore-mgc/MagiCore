use mgc_types::adapter::Vulnerability;

/// Parsed pip-audit result: flattened findings + dependency count audited.
/// Kết quả parse pip-audit: finding + số dependency đã audit.
pub struct PipAuditParse {
    pub vulnerabilities: Vec<Vulnerability>,
    pub packages_audited: usize,
}
