//! Registry-scoped cache identities.
//! Định danh cache theo registry.
//!
//! A cache entry written while installing from one registry must never be
//! served to a project resolving the same `name@version` from another
//! registry: the bytes are only identified by (name, version) and two
//! registries are free to publish different tarballs under the same id.
//! Entry cache ghi khi cài từ registry này không bao giờ được phục vụ cho
//! project resolve cùng `name@version` từ registry khác: bytes chỉ được định
//! danh bằng (name, version) và hai registry có thể publish tarball khác nhau.
//!
//! The scope is a BLAKE3 digest of the normalised registry origin, so the
//! directory name never contains a raw URL (which can hold credentials).
//! Scope là digest BLAKE3 của origin registry đã chuẩn hoá, nên tên thư mục
//! không bao giờ chứa URL thô (có thể chứa credential).

use std::path::{Path, PathBuf};

const REGISTRY_SCOPE_NAMESPACE: &str = ".mgc-cache-registry-v1";

/// Normalise a registry URL to its origin before hashing it.
/// Chuẩn hoá URL registry về origin trước khi hash.
///
/// Scheme, host and port are the security-relevant parts; path components
/// (`/-/`, trailing slashes, `.git`) do not identify a different publisher.
/// Scheme, host và port là phần có ý nghĩa bảo mật; phần path không định
/// danh nhà phát hành khác.
fn normalise_registry_origin(registry: &str) -> String {
    let trimmed = registry.trim();
    let Some((scheme, rest)) = trimmed.split_once("://") else {
        return trimmed.to_ascii_lowercase();
    };
    let scheme = scheme.to_ascii_lowercase();
    let authority_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let authority = &rest[..authority_end];
    // Drop any userinfo component: `user:pass@host` is not part of identity.
    // Bỏ userinfo: `user:pass@host` không thuộc identity.
    let authority = match authority.rsplit_once('@') {
        Some((_, host)) => host,
        None => authority,
    };
    format!("{scheme}://{}", authority.to_ascii_lowercase())
}

pub(in crate::cache) fn registry_scope_directory(root: &Path, registry: &str) -> PathBuf {
    let origin = normalise_registry_origin(registry);
    let digest = blake3::hash(origin.as_bytes()).to_hex().to_string();
    root.join(REGISTRY_SCOPE_NAMESPACE).join(digest)
}
