//! Split a package URL into the existing package name and affected version.
//! Tách package URL thành tên package và version bị ảnh hưởng như hiện tại.

use mgc_types::{PackageName, Version};
/// Split a purl like `pkg:maven/com.example/lib@1.0` into (name, version).
/// The version after '@' is the AFFECTED version — carrying it makes the
/// finding actionable instead of a fake 0.0.0.
/// Tách purl `pkg:maven/com.example/lib@1.0` thành (name, version).
/// Version sau '@' là version BỊ ẢNH HƯỞNG — giữ nó làm finding có ích,
/// không thay bằng 0.0.0 giả.
#[cfg(test)]
pub(in crate::audit::scanner::kotlin::owasp_parser) fn purl_name_version(
    purl: &str,
) -> (PackageName, Option<Version>) {
    let (id_part, version) = match purl.rsplit_once('@') {
        Some((id, v)) => (id, Version::parse(v).ok()),
        None => (purl, None),
    };
    // Artifact tail may contain '::' qualifiers (pkg:npm/name@1.0?qualifier)
    // — strip them before taking the tail.
    let artifact = id_part
        .rsplit('/')
        .next()
        .unwrap_or("artifact")
        .split('?')
        .next()
        .unwrap_or("artifact");
    let name = PackageName::new(artifact)
        .unwrap_or_else(|_| PackageName::new("artifact").expect("static fallback name is valid"));
    (name, version)
}
