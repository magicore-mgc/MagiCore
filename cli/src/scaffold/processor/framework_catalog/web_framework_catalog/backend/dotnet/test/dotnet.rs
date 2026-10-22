//! Tests for the grouped framework configuration rows.
//! Test các row cấu hình framework đã được nhóm.

use super::*;

#[test]
fn dotnet_backend_frameworks_keep_their_language_base() {
    let rows = [DOTNET_MINIMAL, DOTNET_WEBAPI];
    assert_eq!(
        rows.iter().map(|row| row.name).collect::<Vec<_>>(),
        vec!["dotnet-minimal", "dotnet-webapi",]
    );
    assert!(
        rows.iter()
            .all(|row| row.sub_type == "backend" && row.base == Some("dotnet"))
    );
}
