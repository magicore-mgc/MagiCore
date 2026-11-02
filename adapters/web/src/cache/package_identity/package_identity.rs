//! Derive deterministic paths and keys for extracted package content.
//! Tạo path và key xác định cho nội dung package đã giải nén.

use crate::cache::SharedWebCache;
use mgc_types::adapter::ResolvedPackage;
use std::path::{Path, PathBuf};

impl SharedWebCache {
    pub fn extracted_package_root(&self, pkg: &ResolvedPackage) -> PathBuf {
        crate::cache::shared_extracted_package_root(&self.root, pkg)
    }
}

pub fn shared_extracted_package_root(root: &Path, pkg: &ResolvedPackage) -> PathBuf {
    let safe_name = pkg.id.name_str().replace('/', "__").replace('@', "");
    let cache_key = extracted_package_cache_key(pkg);
    root.join("packages")
        .join(safe_name)
        .join(cache_key)
        .join("package")
}

pub fn extracted_package_cache_key(pkg: &ResolvedPackage) -> String {
    // EN: SEC-01 FIX: Include registry in cache key to prevent cache poisoning
    // VI: Sửa SEC-01: Bao gồm registry trong cache key để ngăn cache poisoning
    //
    // EN: Extract registry from tarball_url (e.g., "https://registry.npmjs.org/..." → "registry.npmjs.org")
    // VI: Trích registry từ tarball_url (ví dụ: "https://registry.npmjs.org/..." → "registry.npmjs.org")
    let registry_host = extract_registry_host(&pkg.tarball_url);
    
    if pkg.integrity.is_empty() {
        return format!("{}@{}", registry_host, pkg.id.version());
    }
    let fs_safe = pkg
        .integrity
        .replace('/', "_")
        .replace('+', "-")
        .replace('=', "");
    format!("{}@{}-{}", registry_host, pkg.id.version(), fs_safe)
}

// EN: Extract registry host from tarball URL for cache key isolation
// VI: Trích host registry từ tarball URL để cách ly cache key
fn extract_registry_host(tarball_url: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    // EN: Parse URL and extract host + port
    // VI: Parse URL và trích host + port
    if let Ok(url) = url::Url::parse(tarball_url) {
        if let Some(host) = url.host_str() {
            // EN: Sanitize host for filesystem:
            // - Replace dots with underscores
            // - Remove/replace brackets (IPv6)
            // - Remove colons (IPv6, but NOT from port separator)
            // - Include port to distinguish registries on different ports
            // VI: Làm sạch host cho filesystem:
            // - Thay dấu chấm bằng gạch dưới
            // - Xóa/thay dấu ngoặc vuông (IPv6)
            // - Xóa dấu hai chấm (IPv6, nhưng KHÔNG phải port separator)
            // - Bao gồm port để phân biệt registries trên ports khác nhau
            let sanitized_host = host
                .replace('.', "_")
                .replace('[', "")
                .replace(']', "")
                .replace(':', "_");
            
            // EN: Include port if non-default
            // VI: Bao gồm port nếu không phải default
            if let Some(port) = url.port() {
                return format!("{}_{}", sanitized_host, port);
            }
            
            return sanitized_host;
        }
    }
    
    // EN: Fallback: hash the full URL if parsing fails
    // VI: Dự phòng: hash toàn bộ URL nếu parse thất bại
    let mut hasher = DefaultHasher::new();
    tarball_url.hash(&mut hasher);
    format!("unknown_{:x}", hasher.finish())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
