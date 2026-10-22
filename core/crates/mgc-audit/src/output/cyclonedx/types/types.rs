//! CycloneDX 1.5 data structures and public format constants.
//! Cấu trúc dữ liệu CycloneDX 1.5 và hằng format public.

use serde::Serialize;

/// CycloneDX spec version emitted by this module.
/// Phiên bản spec CycloneDX mà module này phát hành.
pub const CYCLONEDX_SPEC_VERSION: &str = "1.5";
/// BOM format tag mandated by the CycloneDX schema.
/// Thẻ bomFormat mà schema CycloneDX bắt buộc.
pub const CYCLONEDX_BOM_FORMAT: &str = "CycloneDX";

#[derive(Serialize)]
pub struct CycloneDxBom {
    #[serde(rename = "bomFormat")]
    pub bom_format: String,
    #[serde(rename = "specVersion")]
    pub spec_version: String,
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    pub version: u32,
    pub metadata: CycloneDxMetadata,
    /// One component per AFFECTED package (not per finding) — a package
    /// hit by two advisories appears once with two vulnerabilities.
    /// Một component cho mỗi package BỊ ẢNH HƯỞNG (không phải mỗi
    /// finding) — package dính 2 advisory xuất hiện 1 lần với 2 vuln.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CycloneDxComponent>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vulnerabilities: Vec<CycloneDxVulnerability>,
}

#[derive(Serialize)]
pub struct CycloneDxMetadata {
    pub timestamp: String,
    pub tools: CycloneDxTools,
}

/// CycloneDX 1.5 `tools` is a STRUCT with `components` (1.4 used an
/// array) — emitted per the 1.5 schema so strict validators accept it.
/// `tools` của CycloneDX 1.5 là STRUCT chứa `components` (1.4 là mảng) —
/// phát hành theo schema 1.5 để validator chặt chấp nhận.
#[derive(Serialize)]
pub struct CycloneDxTools {
    pub components: Vec<CycloneDxTool>,
}

#[derive(Serialize)]
pub struct CycloneDxTool {
    #[serde(rename = "type")]
    pub tool_type: &'static str,
    pub vendor: &'static str,
    pub name: &'static str,
    pub version: String,
    /// UNVERIFIED marker rides here on non-Available states — loud, not
    /// silent (RULE §11: escape/limit states always announce themselves).
    /// Đánh dấu UNVERIFIED gắn tại đây khi non-Available — rõ ràng, không
    /// im lặng (RULE §11: trạng thái giới hạn luôn tự báo).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<CycloneDxProperty>,
}

#[derive(Serialize)]
pub struct CycloneDxComponent {
    #[serde(rename = "type")]
    pub component_type: &'static str,
    #[serde(rename = "bom-ref")]
    pub bom_ref: String,
    pub name: String,
    pub version: String,
    pub purl: String,
}

#[derive(Serialize)]
pub struct CycloneDxVulnerability {
    #[serde(rename = "bom-ref")]
    pub bom_ref: String,
    pub id: String,
    pub source: CycloneDxSource,
    pub ratings: Vec<CycloneDxRating>,
    pub description: String,
    /// Affected component bom-refs — always non-empty: a vulnerability
    /// row without an affected component is a contract violation.
    /// Bom-ref của các component dính — luôn khác rỗng: dòng vulnerability
    /// mà không có component nào dính là vi phạm hợp đồng.
    pub affects: Vec<CycloneDxAffect>,
    pub properties: Vec<CycloneDxProperty>,
}

#[derive(Serialize)]
pub struct CycloneDxSource {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Serialize)]
pub struct CycloneDxRating {
    /// CycloneDX 1.5 severity names: none, info, low, medium, high,
    /// critical — same five labels as our contract.
    /// Tên severity CycloneDX 1.5 — trùng năm nhãn của hợp đồng ta.
    pub severity: String,
}

#[derive(Serialize)]
pub struct CycloneDxAffect {
    pub r#ref: String,
}

#[derive(Serialize)]
pub struct CycloneDxProperty {
    pub name: String,
    pub value: String,
}
