//! Install policy error contract regression.
//! Test hồi quy contract lỗi install policy.

use super::*;

#[test]
fn unreadable_caller_policy_is_typed_without_echoing_policy_contents() {
    let error = registry_package_invalid_caller_policy();
    assert_eq!(
        error.to_string(),
        "package runner: cannot read caller project policy; fix mgc.toml before running the package"
    );
    assert!(!error.to_string().contains("TOKEN"));
    assert!(matches!(
        error.downcast_ref::<InstallPolicyError>(),
        Some(InstallPolicyError::CallerPolicyInvalid)
    ));
}
