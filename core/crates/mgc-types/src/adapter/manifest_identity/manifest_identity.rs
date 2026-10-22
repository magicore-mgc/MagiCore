//! Value identity used to fence mutation journals to one manifest lane. — Identity theo value dùng để giới hạn mutation journal vào đúng lane manifest.

/// Owner identity of one natively-mutable manifest lane. Compared by
/// VALUE in crash recovery — every field must match.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ManifestIdentity {
    /// Adapter core name: web/lib/app/game/iot/...
    pub core: String,
    /// Lane language/framework: python, bevy, flutter, terraform, ...
    pub language: String,
    /// Manifest format filename or pattern: pyproject.toml, *.uproject, ...
    pub format: String,
    /// Normalized manifest path relative to the project root.
    pub relpath: String,
}
