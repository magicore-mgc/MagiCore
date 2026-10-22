//! Resolve the authoritative registration catalog SHA-512 marker.
//! Resolve marker SHA-512 catalog registration chính thống.

use crate::protocols::nuget::NuGetProtocol;
use base64::Engine as _;
use mgc_types::{MgError, MgResult};
use serde_json::Value;

/// Follow a listed registration leaf to its catalog document and validate its SHA-512 value.
/// Theo registration leaf đang listed tới catalog và kiểm tra giá trị SHA-512.
pub(in crate::protocols::nuget::resolve) async fn catalog_sha512_marker(
    protocol: &NuGetProtocol,
    id: &str,
    id_lower: &str,
    version: &str,
) -> MgResult<String> {
    // Registration leaf for the selected version is authoritative.
    // Registration leaf của version được chọn là nguồn chính thống.
    let leaves = protocol.registration_leaves(id_lower).await?;
    let leaf = leaves.iter().find(|leaf| {
        leaf.get("catalogEntry")
            .and_then(|catalog| catalog.get("version"))
            .and_then(Value::as_str)
            .is_some_and(|candidate| candidate.eq_ignore_ascii_case(version))
    });
    let catalog = leaf
        .and_then(|leaf| leaf.get("catalogEntry"))
        .ok_or_else(|| {
            MgError::Integrity(format!(
                "registration has no catalogEntry for {id} {version} — artifact cannot be \
                 verified (fail-closed)"
            ))
        })?;
    let listed = catalog
        .get("listed")
        .and_then(Value::as_bool)
        .unwrap_or(true);
    if !listed {
        return Err(MgError::Integrity(format!(
            "{id} {version} is unlisted in the registration (fail-closed)"
        )));
    }

    // The registration projection omits packageHash, so follow catalog @id.
    // Registration projection thiếu packageHash nên phải theo catalog @id.
    let catalog_url = catalog.get("@id").and_then(Value::as_str).ok_or_else(|| {
        MgError::Integrity(format!(
            "{id} {version} catalogEntry has no @id — hash unreachable (fail-closed)"
        ))
    })?;
    let (status, body) = protocol.get_text(catalog_url).await?;
    if !(200..300).contains(&status) {
        return Err(MgError::Network(format!(
            "GET {catalog_url} returned {status} — catalog hash unreachable (fail-closed)"
        )));
    }
    let doc: Value = serde_json::from_str(&body)
        .map_err(|error| MgError::Other(format!("parse catalog leaf failed: {error}")))?;
    let package_hash = doc
        .get("packageHash")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let algorithm = doc
        .get("packageHashAlgorithm")
        .and_then(Value::as_str)
        .unwrap_or_default();

    validate_catalog_hash(id, version, algorithm, package_hash)
}

/// Validate the catalog algorithm and base64 syntax, returning the unchanged marker spelling.
/// Kiểm tra algorithm và cú pháp base64, trả marker giữ nguyên spelling.
fn validate_catalog_hash(
    id: &str,
    version: &str,
    algorithm: &str,
    package_hash: &str,
) -> MgResult<String> {
    if !algorithm.eq_ignore_ascii_case("SHA512") || package_hash.is_empty() {
        return Err(MgError::Integrity(format!(
            "{id} {version} catalog hash algorithm '{algorithm}' is not SHA512 or the hash \
             is empty — fail-closed"
        )));
    }
    base64::engine::general_purpose::STANDARD
        .decode(package_hash)
        .map_err(|error| {
            MgError::Integrity(format!(
                "{id} {version} packageHash is not valid base64: {error}"
            ))
        })?;
    Ok(format!("sha512:{package_hash}"))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
