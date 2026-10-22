//! Checks the empty v3 graph conversion contract.
//! Kiểm tra hợp đồng chuyển đồ thị v3 rỗng.

use super::super::migrate_v3_to_v4;
use crate::Lockfile;

#[test]
fn converts_an_empty_v3_lock_without_inventing_packages() {
    let (document, warnings) =
        migrate_v3_to_v4(Lockfile::new()).expect("test operation must succeed");
    assert!(document.packages.is_empty());
    assert!(warnings.is_empty());
}
