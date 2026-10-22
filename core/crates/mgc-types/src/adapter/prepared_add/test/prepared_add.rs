//! Prepared Add owner contract test.
//! Kiểm tra hợp đồng dữ liệu chuẩn bị thêm package.

use crate::adapter::PreparedAdd;
use crate::package::{PackageId, VersionRange};

#[test]
fn prepared_add_round_trip_preserves_selected_identity_and_range() {
    let prepared = PreparedAdd {
        id: PackageId::parse("sample@1.2.3").unwrap(),
        range: VersionRange::parse("^1.2.0").unwrap(),
    };
    let json = serde_json::to_string(&prepared).unwrap();
    let decoded: PreparedAdd = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.id, prepared.id);
    assert_eq!(decoded.range, prepared.range);
}
