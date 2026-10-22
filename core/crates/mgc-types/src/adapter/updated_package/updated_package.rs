//! Before/after package version record shared by update reporting. — Bản ghi phiên bản package trước/sau dùng cho báo cáo update.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdatedPackage {
    pub name: String,
    pub from_version: String,
    pub to_version: String,
}
