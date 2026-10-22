//! Verify the local conflict error has a stable display and downcast type.
//! Kiểm tra lỗi xung đột có thông báo ổn định và downcast được.

use super::CoreOverrideConflictError;

#[test]
fn core_override_conflict_preserves_the_existing_user_message() {
    let typed = CoreOverrideConflictError::new("ai", "web");
    assert_eq!(
        typed.to_string(),
        "core override 'ai' conflicts with existing project core 'web'; core reassignment is not supported by this version"
    );

    let wrapped = anyhow::Error::new(typed);
    assert!(
        wrapped
            .downcast_ref::<CoreOverrideConflictError>()
            .is_some()
    );
}
