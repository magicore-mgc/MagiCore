//! Select a trusted package tarball URL or the registry fallback.
//! Chọn URL tarball được tin cậy hoặc đường dẫn fallback của registry.

use mgc_types::adapter::ResolvedPackage;

use crate::audit::{allow_insecure_loopback_url, is_tarball_url_trusted};

/// Construct tarball download URL for a package.
/// Xây dựng URL tải tarball cho package — ưu tiên pkg.tarball_url, fallback registry chuẩn.
///
/// Validates HTTPS (except loopback) and domain trust before using pkg.tarball_url.
/// Kiểm tra HTTPS (trừ loopback) và trust domain trước khi dùng pkg.tarball_url.
pub fn package_tarball_url(registry_url: &str, pkg: &ResolvedPackage) -> String {
    let fallback = |_registry: &str, _pkg: &ResolvedPackage| {
        format!(
            "{}/{}/-/{}-{}.tgz",
            _registry.trim_end_matches('/'),
            _pkg.id.name_str(),
            _pkg.id.name().unscoped(),
            _pkg.id.version()
        )
    };

    // If package has explicit tarball URL, validate before using
    // Nếu package có tarball URL rõ ràng, validate trước khi dùng
    if !pkg.tarball_url.is_empty() {
        // Reject non-HTTPS (except localhost/loopback for testing)
        // Reject không-HTTPS (trừ localhost/loopback cho testing)
        if !pkg.tarball_url.starts_with("https://")
            && !allow_insecure_loopback_url(&pkg.tarball_url)
        {
            eprintln!(
                "WARNING: Tarball URL for '{}' is not HTTPS, using registry fallback",
                pkg.id.name_str()
            );
            return fallback(registry_url, pkg);
        }

        // Check domain matches registry (prevent tarball hijack)
        // Kiểm tra domain khớp registry (chống hijack tarball)
        if !is_tarball_url_trusted(&pkg.tarball_url, registry_url) {
            eprintln!(
                "WARNING: Tarball URL for '{}' domain mismatch with registry, using registry fallback",
                pkg.id.name_str()
            );
            return fallback(registry_url, pkg);
        }

        return pkg.tarball_url.clone();
    }

    // No explicit URL: use registry default pattern
    // Không có URL rõ: dùng pattern registry mặc định
    fallback(registry_url, pkg)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
