//! Canonicalize ecosystem names, versions and peer contexts.
//! Chuẩn hóa tên/version hệ sinh thái và peer context.

use crate::ecosystem_tag::EcosystemTag;

/// Canonical package name per ecosystem.
/// - npm: lowercase.
/// - PyPI: PEP 503 (`[-_.]+` → `-`, lowercase).
/// - NuGet: lowercase.
/// - everything else: trimmed as-is.
///
/// Tên canonical theo ecosystem.
pub fn canonical_name(ecosystem: EcosystemTag, raw: &str) -> String {
    let trimmed = raw.trim();
    match ecosystem {
        EcosystemTag::Web | EcosystemTag::NuGet => trimmed.to_ascii_lowercase(),
        EcosystemTag::Python => {
            let mut out = String::with_capacity(trimmed.len());
            let mut last_dash = false;
            for ch in trimmed.chars() {
                if ch == '-' || ch == '_' || ch == '.' {
                    if !last_dash {
                        out.push('-');
                    }
                    last_dash = true;
                } else {
                    for lower in ch.to_lowercase() {
                        out.push(lower);
                    }
                    last_dash = false;
                }
            }
            out
        }
        _ => trimmed.to_string(),
    }
}

/// Canonical version string: `1.2` → `1.2.0`, `1` → `1.0.0`; longer
/// versions and pre-release/build suffixes pass through unchanged.
/// Chuỗi version canonical.
pub fn canonical_version(raw: &str) -> String {
    let trimmed = raw.trim();
    let (core, suffix) = match trimmed.find(['-', '+']) {
        Some(idx) => trimmed.split_at(idx),
        None => (trimmed, ""),
    };
    let parts: Vec<&str> = core.split('.').collect();
    let normalized = match parts.as_slice() {
        [major] => format!("{major}.0.0"),
        [major, minor] => format!("{major}.{minor}.0"),
        _ => core.to_string(),
    };
    format!("{normalized}{suffix}")
}

/// Digest of a resolved peer set: `blake3(canonical_join(sorted(
/// [(peer_name, resolved_version), …])))`, hex-encoded. The lock also
/// stores the canonical `peer_contexts` table so verify/replay/audit
/// recompute the digest instead of trusting it blindly.
/// Digest của tập peer đã chốt. Lock còn lưu bảng `peer_contexts`
/// canonical để verify tính lại, không tin mù.
pub fn peer_context_digest(peers: &[(String, String)]) -> String {
    use mgc_crypto::blake3_signer::Blake3Hasher;
    let mut sorted: Vec<(&str, &str)> = peers
        .iter()
        .map(|(n, v)| (n.as_str(), v.as_str()))
        .collect();
    sorted.sort();
    let joined = sorted
        .iter()
        .map(|(n, v)| format!("{n}@{v}"))
        .collect::<Vec<_>>()
        .join(",");
    Blake3Hasher::hash_string(&joined).to_hex()
}

#[cfg(test)]
#[path = "test/canonical_identity.rs"]
mod tests;
