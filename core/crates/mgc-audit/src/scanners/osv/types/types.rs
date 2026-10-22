//! OSV package and response wire types..
//! Định nghĩa OsvPin công khai và DTO phản hồi OSV nội bộ.

use serde::Deserialize;

/// One pinned dependency to query.
/// Một dependency ghim để truy vấn.
#[derive(Debug, Clone)]
pub struct OsvPin {
    pub name: String,
    pub version: String,
    pub ecosystem: &'static str,
}

/// Query response: OSV ids for ONE package+version.
/// Response query: các id OSV cho MỘT package+version.
#[derive(Debug, Deserialize)]
pub(crate) struct OsvQueryResponse {
    #[serde(default)]
    pub(in crate::scanners::osv) vulns: Vec<OsvIdOnly>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OsvIdOnly {
    pub(in crate::scanners::osv) id: String,
}

/// Vuln detail (subset of the OSV schema we consume).
/// Chi tiết vuln (tập con schema OSV ta dùng).
#[derive(Debug, Deserialize)]
pub(crate) struct OsvVuln {
    pub(in crate::scanners::osv) id: String,
    #[serde(default)]
    pub(in crate::scanners::osv) summary: String,
    #[serde(default)]
    pub(in crate::scanners::osv) details: String,
    #[serde(default)]
    pub(in crate::scanners::osv) aliases: Vec<String>,
    #[serde(default)]
    pub(in crate::scanners::osv) severity: Vec<OsvSeverity>,
    #[serde(default, rename = "database_specific")]
    pub(in crate::scanners::osv) database_specific: Option<OsvDbSpecific>,
    #[serde(default)]
    pub(crate) references: Vec<OsvReference>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OsvSeverity {
    #[serde(default)]
    pub(in crate::scanners::osv) score: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    pub(crate) r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OsvDbSpecific {
    #[serde(default)]
    pub(in crate::scanners::osv) severity: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OsvReference {
    #[serde(default)]
    pub(in crate::scanners::osv) url: Option<String>,
}
