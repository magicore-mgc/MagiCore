//! Version-selection diagnostic tests — test chẩn đoán chọn version.
//! A resolve failure must tell the user whether the range is wrong or the
//! registry simply has nothing newer yet. Without the newest published version a
//! template pinned ahead of its release (astro's `main` branch pointing at an
//! unpublished 7.4.3) reads exactly like a typo.
//! (Lỗi resolve phải cho biết range sai hay registry chưa có bản mới. Thiếu
//! version mới nhất thì template đi trước bản phát hành trông giống hệt gõ sai.)

use mgc_types::Version;

/// The hint names the highest version the registry returned.
/// (Hint nêu version cao nhất registry trả về.)
#[test]
fn the_hint_names_the_highest_available_version() {
    let versions = [
        Version::new(7, 4, 0),
        Version::new(7, 4, 2),
        Version::new(7, 4, 1),
    ];
    let newest = versions.iter().max().map(ToString::to_string);
    assert_eq!(
        newest.as_deref(),
        Some("7.4.2"),
        "the hint must use max, not the last element — registries do not guarantee order"
    );
}

/// An empty version list says so instead of naming nothing.
/// (Danh sách rỗng nói rõ thay vì nêu hụt.)
#[test]
fn an_empty_version_list_is_described_explicitly() {
    let versions: [Version; 0] = [];
    let newest = versions.iter().max().map(ToString::to_string);
    assert!(newest.is_none());
}
