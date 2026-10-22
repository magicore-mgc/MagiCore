//! Carries only verifiable digests into v4 artifact metadata.
//! Chỉ chuyển digest có thể xác minh sang metadata artifact v4.

use crate::Package;

// Convert the native V3 SHA-256 hex representation without calculating a new hash.
// Chỉ đổi mã hóa của cùng 32 byte digest; không tạo bằng chứng artifact mới.
fn native_sha256_sri(integrity: &str) -> Option<String> {
    use base64::Engine;
    let raw = integrity.strip_prefix("sha256-")?;
    if raw.len() != 64 {
        return None;
    }
    let digest = hex::decode(raw).ok()?;
    Some(format!(
        "sha256-{}",
        base64::engine::general_purpose::STANDARD.encode(digest)
    ))
}

/// Preserve supported SRI or normalize the native V3 SHA-256 hex encoding.
/// Encoding conversion retains the existing digest; it never calculates a hash.
/// Unsupported digests are refused with warnings. A missing artifact uses the
/// pin's own URL/registry; pins without an artifact URL remain unavailable.
/// Giữ SRI hợp lệ hoặc đổi mã hóa digest SHA-256 hex V3 mà không tạo hash mới.
/// Marker không hỗ trợ vẫn cảnh báo; artifact chỉ dùng URL/registry sẵn có của pin.
pub(in crate::migrate::v3_to_v4) fn carry_install_sri(
    package: &Package,
    warnings: &mut Vec<String>,
) -> Option<crate::schema::ArtifactRef> {
    let mut artifact = package
        .artifact
        .clone()
        .unwrap_or_else(|| crate::schema::ArtifactRef {
            url: package.resolved.clone(),
            size_bytes: None,
            content_hash: String::new(),
            downloaded_from: package.registry.clone().unwrap_or_default(),
            integrity_sri: None,
        });
    if artifact.url.is_empty() {
        warnings.push(format!(
            "pin {}@{} has no artifact URL: v4 install could never fetch it",
            package.name, package.version
        ));
        return None;
    }
    if crate::policy::check_install_sri(&package.integrity).is_ok() {
        artifact.integrity_sri = Some(package.integrity.clone());
    } else if let Some(sri) = native_sha256_sri(&package.integrity) {
        artifact.integrity_sri = Some(sri);
    } else if !package.integrity.is_empty() {
        warnings.push(format!(
            "pin {}@{} has SRI '{}' the installer cannot enforce: dropped from v4 artifact (re-resolve to refresh)",
            package.name, package.version, package.integrity
        ));
    }
    Some(artifact)
}
