use crate::allowlist::reject_forbidden_pm_script;

#[test]
fn preserves_clear_rejection_for_external_package_manager_delegation() {
    let error = reject_forbidden_pm_script("npm install").unwrap_err();
    assert!(
        error
            .to_string()
            .contains("forbidden package manager 'npm'")
    );
    assert!(reject_forbidden_pm_script("vite build").is_ok());
}
