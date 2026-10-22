use mgc_types::adapter::Vulnerability;

/// Parsed cargo-audit result: findings + the real dependency count.
/// Kết quả parse cargo-audit: finding + số dependency thật đã audit.
pub struct CargoAuditParse {
    pub vulnerabilities: Vec<Vulnerability>,
    pub packages_audited: usize,
}
