// Public display compatibility test.
// Test tương thích chuỗi hiển thị công khai.
use super::super::MergeConflict;

#[test]
fn conflict_display_preserves_legacy_message() {
    let conflict = MergeConflict {
        name: "shared".to_string(),
        base_version: None,
        ours_version: "1.0.0".to_string(),
        theirs_version: "2.0.0".to_string(),
    };

    assert_eq!(
        conflict.to_string(),
        "conflict: package 'shared' changed on both sides (base <none>, ours 1.0.0, theirs 2.0.0)"
    );
}
