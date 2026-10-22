//! Language projection tests — test projection language.
//! Locks the differing Lib language rows and their manifest condition.
//! Khóa các hàng language khác biệt của Lib và điều kiện manifest.

use super::super::language_projection;
use crate::commands::capabilities::dependency_ownership::operation_projection::operations_for;

#[test]
fn lib_language_rows_keep_the_manifest_qualified_native_lanes() {
    let base = operations_for("lib", None, None);
    let languages = language_projection("lib", &base);

    assert_eq!(languages["ts"]["add"]["owner"], "mgc-native");
    assert_eq!(languages["rust"]["install"]["owner"], "mgc-native");
    assert_eq!(languages["python"]["add"]["owner"], "mgc-native");
    assert_eq!(languages["go"]["remove"]["owner"], "mgc-native");
    assert!(
        languages["ts"]["add"]["tools"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}
