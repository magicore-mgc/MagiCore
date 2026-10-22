//! Owns the reserved Unity dependency projection, still unwired until native lock writing exists. — Sở hữu projection dependency Unity đang để dành, chưa nối tới khi có ghi lock native.

/// Declared Unity packages from `Packages/manifest.json` (`dependencies` keys).
/// (Package Unity khai báo từ `Packages/manifest.json` (key `dependencies`).)
///
/// Reserved: Unity lanes have no native `mgc.lock` writer yet (game adapter
/// only reads locks for SBOM), so wiring this would dead-end: the check
/// could never pass and `mgc install` could never fix it.
/// (Dành sẵn: lane Unity chưa ghi `mgc.lock` native (adapter game chỉ đọc
/// lock cho SBOM), nên nối sẽ bế tắc: check không bao giờ qua và
/// `mgc install` không bao giờ sửa được.)
#[allow(dead_code)]
pub fn declared_unity_deps(manifest_json: &str) -> Vec<String> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(manifest_json) else {
        return Vec::new();
    };
    let mut names: Vec<String> = value
        .get("dependencies")
        .and_then(|v| v.as_object())
        .map(|deps| deps.keys().cloned().collect())
        .unwrap_or_default();
    names.sort();
    names.dedup();
    names
}
