//! Swift Package.resolved v1/v2 wire structures..
//! Định nghĩa schema hai phiên bản Package.resolved.

use serde::Deserialize;

/// Package.resolved v2 (Xcode 14+): {"pins": [{"identity", "state":
/// {"version"}}], "version": 2}; v1: {"object": {"pins": [...]}}.
/// Both shapes parse into the same pins.
/// Package.resolved v2: pins trực tiếp; v1: pins trong object — cả hai
/// về cùng một tập ghim.
#[derive(Debug, Deserialize)]
pub(crate) struct SwiftResolved {
    #[serde(default)]
    pub(in crate::scanners::osv::swift) pins: Vec<SwiftPin>,
    #[serde(default)]
    pub(in crate::scanners::osv::swift) object: Option<SwiftResolvedV1Object>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SwiftResolvedV1Object {
    #[serde(default)]
    pub(in crate::scanners::osv::swift) pins: Vec<SwiftPinV1>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SwiftPin {
    #[serde(default)]
    pub(in crate::scanners::osv::swift) identity: String,
    /// v2 pins carry the git location (OSV `SwiftURL` names are git URLs).
    /// Ghim v2 kèm vị trí git (name `SwiftURL` của OSV là URL git).
    #[serde(default)]
    pub(in crate::scanners::osv::swift) location: Option<String>,
    #[serde(default)]
    pub(in crate::scanners::osv::swift) state: Option<SwiftPinState>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SwiftPinState {
    #[serde(default)]
    pub(in crate::scanners::osv::swift) version: Option<String>,
    #[serde(default)]
    pub(in crate::scanners::osv::swift) branch: Option<String>,
    #[serde(default)]
    pub(crate) revision: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SwiftPinV1 {
    #[serde(default)]
    pub(in crate::scanners::osv::swift) package: String,
    /// v1 pins carry repositoryURL (same git-URL naming contract).
    /// Ghim v1 kèm repositoryURL (cùng hợp đồng đặt tên URL git).
    #[serde(default, rename = "repositoryURL")]
    pub(crate) repository_url: Option<String>,
    #[serde(default)]
    pub(in crate::scanners::osv::swift) state: Option<SwiftPinState>,
}
