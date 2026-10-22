use super::super::super::LauncherPolicyError;

#[test]
fn test_dangerous_flag_error_preserves_display() {
    let error = LauncherPolicyError::DangerousFlag {
        runtime: "bun",
        flag: "--eval",
    };
    assert_eq!(
        error.to_string(),
        "Rejected dangerous bun flag: --eval. Use project scripts only."
    );
}

#[test]
fn test_dangerous_permission_error_preserves_display() {
    let error = LauncherPolicyError::DangerousPermission {
        runtime: "deno",
        permission: "--allow-read",
    };
    assert_eq!(
        error.to_string(),
        "Rejected dangerous deno permission: --allow-read. Explicitly allow in deno.json tasks if needed."
    );
}
