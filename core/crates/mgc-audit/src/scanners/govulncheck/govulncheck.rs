//! Public Go parser result type.
//! (Kiểu kết quả parser Go được công khai.)

use mgc_types::adapter::Vulnerability;

/// Parsed govulncheck stream: deduped findings + module count.
/// Kết quả parse stream govulncheck: finding khử trùng + số module.
pub struct GovulncheckParse {
    pub vulnerabilities: Vec<Vulnerability>,
    pub packages_audited: usize,
}
