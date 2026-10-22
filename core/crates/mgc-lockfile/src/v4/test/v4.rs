//! Keep the legacy and additive v4 revisions readable.
//! Giữ khả năng đọc revision v4 cũ và revision cộng thêm.

#[test]
fn accepts_only_the_supported_v4_revision_strings() {
    assert!(super::is_v4_schema_version("4"));
    assert!(super::is_v4_schema_version("4.1"));
    assert!(!super::is_v4_schema_version("3"));
    assert!(!super::is_v4_schema_version("4.2"));
}
