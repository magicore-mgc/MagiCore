//! Checks the v2-to-v3 schema and provenance transition.
//! Kiểm tra chuyển schema và provenance từ v2 sang v3.

use super::super::migrate_v2_to_v3;
use crate::{Lockfile, Package};

#[test]
fn fills_missing_provenance() {
    let mut lock = Lockfile::new();
    lock.version = "2".to_string();
    lock.packages.push(Package::new(
        "sample".into(),
        "1.0.0".into(),
        "https://example.invalid/sample.tgz".into(),
        String::new(),
    ));
    let migrated = migrate_v2_to_v3(lock);
    assert_eq!(migrated.version, "3");
    assert_eq!(
        migrated.packages[0]
            .provenance
            .as_ref()
            .expect("test operation must succeed")
            .source_kind,
        "registry-import"
    );
}
