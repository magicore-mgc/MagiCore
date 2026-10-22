//! Ensures missing v1 digests remain missing after migration.
//! Đảm bảo digest v1 bị thiếu vẫn được giữ là thiếu sau migration.

use super::super::migrate_v1_to_v2;
use crate::migrate::{LockfileV1, PackageV1};

#[test]
fn does_not_fabricate_integrity_from_the_resolved_url() {
    let migrated = migrate_v1_to_v2(LockfileV1 {
        version: "1".to_string(),
        packages: vec![PackageV1 {
            name: "sample".to_string(),
            version: "1.0.0".to_string(),
            resolved: "https://example.invalid/sample.tgz".to_string(),
            dependencies: Vec::new(),
        }],
    })
    .expect("test operation must succeed");
    assert!(migrated.packages[0].integrity.is_empty());
}
