//! SEC-01: Cache poisoning prevention tests
//! Test ngăn chặn cache poisoning SEC-01

use crate::cache::extracted_package_cache_key;
use mgc_types::adapter::ResolvedPackage;
use mgc_types::package::{PackageId, PackageName};
use mgc_types::version::Version;

// EN: Test that different registries produce different cache keys
// VI: Test rằng các registry khác nhau tạo ra cache key khác nhau
#[test]
fn test_different_registries_different_cache_keys() {
    // EN: Same package name and version, but different registries
    // VI: Cùng tên package và version, nhưng khác registry
    
    let pkg_npm = ResolvedPackage {
        id: PackageId::new(
            PackageName::new("lodash").unwrap(),
            Version::new(4, 17, 21)
        ),
        integrity: "sha512-v2kDEe57lecTulaDIuNTPy3Ry4gLGJ6Z1O3vE1krgXZNrsQ+LFTGHVxVjcXPs17LhbZVGedAJv8XZ1tvj5FvSg==".to_string(),
        tarball_url: "https://registry.npmjs.org/lodash/-/lodash-4.17.21.tgz".to_string(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    };
    
    let pkg_custom = ResolvedPackage {
        id: PackageId::new(
            PackageName::new("lodash").unwrap(),
            Version::new(4, 17, 21)
        ),
        integrity: "sha512-v2kDEe57lecTulaDIuNTPy3Ry4gLGJ6Z1O3vE1krgXZNrsQ+LFTGHVxVjcXPs17LhbZVGedAJv8XZ1tvj5FvSg==".to_string(),
        tarball_url: "https://custom-registry.com/lodash/-/lodash-4.17.21.tgz".to_string(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    };
    
    let key_npm = extracted_package_cache_key(&pkg_npm);
    let key_custom = extracted_package_cache_key(&pkg_custom);
    
    // EN: Keys MUST be different to prevent cache poisoning
    // VI: Các key PHẢI khác nhau để ngăn cache poisoning
    assert_ne!(
        key_npm, key_custom,
        "Cache keys for same package from different registries must differ"
    );
    
    // EN: Verify npm registry key contains "registry_npmjs_org"
    // VI: Xác minh key của npm registry chứa "registry_npmjs_org"
    assert!(
        key_npm.contains("registry_npmjs_org"),
        "npm registry key should contain sanitized host: {}",
        key_npm
    );
    
    // EN: Verify custom registry key contains "custom-registry_com"
    // VI: Xác minh key của custom registry chứa "custom-registry_com"
    assert!(
        key_custom.contains("custom-registry_com"),
        "custom registry key should contain sanitized host: {}",
        key_custom
    );
}

// EN: Test that same registry produces same key (idempotent)
// VI: Test rằng cùng registry tạo ra cùng key (idempotent)
#[test]
fn test_same_registry_same_cache_key() {
    let pkg1 = ResolvedPackage {
        id: PackageId::new(
            PackageName::new("react").unwrap(),
            Version::new(18, 2, 0)
        ),
        integrity: "sha512-example".to_string(),
        tarball_url: "https://registry.npmjs.org/react/-/react-18.2.0.tgz".to_string(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    };
    
    let pkg2 = ResolvedPackage {
        id: PackageId::new(
            PackageName::new("react").unwrap(),
            Version::new(18, 2, 0)
        ),
        integrity: "sha512-example".to_string(),
        tarball_url: "https://registry.npmjs.org/react/-/react-18.2.0.tgz".to_string(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    };
    
    assert_eq!(
        extracted_package_cache_key(&pkg1),
        extracted_package_cache_key(&pkg2),
        "Same package from same registry should produce identical cache keys"
    );
}

// EN: Test cache key format is filesystem-safe
// VI: Test format cache key an toàn cho filesystem
#[test]
fn test_cache_key_filesystem_safe() {
    let pkg = ResolvedPackage {
        id: PackageId::new(
            PackageName::new("test").unwrap(),
            Version::new(1, 0, 0)
        ),
        integrity: "sha512-test+value/unsafe=".to_string(),
        tarball_url: "https://registry.npmjs.org/test/-/test-1.0.0.tgz".to_string(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    };
    
    let key = extracted_package_cache_key(&pkg);
    
    // EN: Verify no filesystem-unsafe characters
    // VI: Xác minh không có ký tự không an toàn cho filesystem
    assert!(
        !key.contains('/'),
        "Cache key must not contain slashes"
    );
    assert!(
        !key.contains('\\'),
        "Cache key must not contain backslashes"
    );
    assert!(
        !key.contains(':'),
        "Cache key must not contain colons (Windows drive letters)"
    );
}

// EN: Test malformed URL fallback
// VI: Test dự phòng URL sai định dạng
#[test]
fn test_malformed_url_fallback() {
    let pkg = ResolvedPackage {
        id: PackageId::new(
            PackageName::new("test").unwrap(),
            Version::new(1, 0, 0)
        ),
        integrity: "sha512-test".to_string(),
        tarball_url: "not-a-valid-url".to_string(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    };
    
    let key = extracted_package_cache_key(&pkg);
    
    // EN: Should still produce a valid key (using hash fallback)
    // VI: Vẫn phải tạo ra key hợp lệ (dùng hash dự phòng)
    assert!(
        key.starts_with("unknown_"),
        "Malformed URL should use fallback hash: {}",
        key
    );
    assert!(
        !key.is_empty(),
        "Cache key must not be empty even with malformed URL"
    );
}
