//! Finding category that distinguishes vulnerabilities, policy, provenance and artifacts. — Nhóm finding phân biệt lỗ hổng, policy, provenance và artifact.

/// Finding class — WHAT KIND of result a Vulnerability row carries (R4).
/// CVE/advisory rows are `Vulnerability`; policy violations (CI workflow
/// rules), provenance/integrity verdicts, and artifact scans (model
/// weights) get their own class so consumers never mistake a policy note
/// for a CVE. Defaults to `Vulnerability` so legacy payloads without the
/// field keep their meaning (backward compatible).
/// Loại finding — dòng Vulnerability mang KẾT QUẢ gì. Mặc định
/// `Vulnerability` để payload cũ thiếu field vẫn đúng nghĩa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FindingClass {
    #[default]
    Vulnerability,
    Policy,
    Provenance,
    Artifact,
}

impl FindingClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vulnerability => "vulnerability",
            Self::Policy => "policy",
            Self::Provenance => "provenance",
            Self::Artifact => "artifact",
        }
    }
}

impl std::fmt::Display for FindingClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
