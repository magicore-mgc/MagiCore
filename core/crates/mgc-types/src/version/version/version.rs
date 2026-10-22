//! Defines the public version value and constructor.
//! Định nghĩa value version công khai và constructor.

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
    /// Semver build metadata, preserved in package identity but ignored by range precedence.
    /// Metadata build semver được giữ trong identity package nhưng bỏ qua khi so range.
    #[serde(default)]
    pub build: Option<String>,
    /// PEP 440 dev release (`1.2.3.dev1`) — sorts below every
    /// pre-release/final/post of the same triple. `None` = not a dev release.
    /// Bản dev PEP 440 xếp trước pre-release/final/post cùng bộ số; `None` = không phải dev.
    #[serde(default)]
    pub dev: Option<u64>,
    /// PEP 440 post release (`1.2.3.post1`) — sorts above the final release
    /// of the same triple. `None` = no post segment.
    /// Bản post PEP 440 xếp sau final cùng bộ số; `None` = không có hậu tố post.
    #[serde(default)]
    pub post: Option<u64>,
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: None,
            build: None,
            dev: None,
            post: None,
        }
    }
}
