//! Define signed release manifest and per-artifact coordinates.
//! Khai báo manifest release có chữ ký và tọa độ từng artifact.

/// Release manifest: binds every artifact of one release to its
/// coordinates + digests. Published as `manifest.json` next to the
/// archives; `manifest.json.sig` carries the detached Ed25519 signature
/// over the exact manifest bytes. Field-by-field binding is verified —
/// a manifest for another version (or a tampered entry) aborts.
/// (Manifest release: bind mọi artifact với tọa độ + digest.)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReleaseManifest {
    /// Release version WITHOUT leading `v` (e.g. `1.1.0-rc.10`).
    pub version: String,
    /// One entry per published archive.
    pub artifacts: Vec<ManifestArtifact>,
}

/// One manifest entry — all coordinates needed to bind an artifact.
/// (Một entry manifest.)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ManifestArtifact {
    /// Package name (`magicore` | `magicore-web`).
    pub package: String,
    /// Contract OS label (`linux` | `macos` | `windows`).
    pub os: String,
    /// Contract arch label (`x64` | `arm64`).
    pub arch: String,
    /// Archive file name (contract format).
    pub archive: String,
    /// Lowercase hex SHA-256 of the archive bytes.
    pub sha256: String,
    /// Archive size in bytes (quota + truncation check).
    pub size: u64,
}
