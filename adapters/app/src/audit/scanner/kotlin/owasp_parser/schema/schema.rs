//! Typed OWASP dependency-check report schema for tests.
//! Schema typed của report OWASP dependency-check dùng trong test.

use serde::Deserialize;

/// Typed OWASP dependency-check report schema — verified against the
/// official JSON reporter format (dependencies[].packages[].package.id
/// purl, vulnerabilities[].cvssv3/cvssv2, projectReportDate etc.).
#[cfg(test)]
#[derive(Debug, Deserialize)]
pub(crate) struct OwaspReport {
    pub(in crate::audit::scanner::kotlin::owasp_parser) dependencies: Vec<OwaspDependency>,
}

#[cfg(test)]
#[derive(Debug, Deserialize)]
pub(crate) struct OwaspDependency {
    #[serde(default)]
    pub(in crate::audit::scanner::kotlin::owasp_parser) packages: Vec<OwaspPackageWrapper>,
    #[serde(default)]
    pub(in crate::audit::scanner::kotlin::owasp_parser) vulnerabilities: Vec<OwaspVulnerability>,
}

#[cfg(test)]
#[derive(Debug, Deserialize)]
pub(crate) struct OwaspPackageWrapper {
    pub(in crate::audit::scanner::kotlin::owasp_parser) package: OwaspPackage,
}

#[cfg(test)]
#[derive(Debug, Deserialize)]
pub(crate) struct OwaspPackage {
    /// Package URL, e.g. `pkg:maven/com.example/lib@1.0`.
    pub(in crate::audit::scanner::kotlin::owasp_parser) id: String,
}

#[cfg(test)]
#[derive(Debug, Deserialize)]
pub(crate) struct OwaspVulnerability {
    pub(in crate::audit::scanner::kotlin::owasp_parser) name: String,
    #[serde(default)]
    pub(in crate::audit::scanner::kotlin::owasp_parser) severity: Option<String>,
    pub(in crate::audit::scanner::kotlin::owasp_parser) cvssv3: Option<OwaspCvss>,
    pub(in crate::audit::scanner::kotlin::owasp_parser) cvssv2: Option<OwaspCvss>,
    #[serde(default)]
    pub(in crate::audit::scanner::kotlin::owasp_parser) description: Option<String>,
}

#[cfg(test)]
#[derive(Debug, Deserialize)]
pub(crate) struct OwaspCvss {
    /// baseScore — may be absent on partial CVSS records.
    /// baseScore — có thể thiếu trên bản ghi CVSS không đầy đủ.
    #[serde(default, rename = "baseScore")]
    pub(in crate::audit::scanner::kotlin::owasp_parser) base_score: Option<f64>,
}
