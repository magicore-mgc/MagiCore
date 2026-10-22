//! Package-run coordinator error contract regression.
//! Test hồi quy contract lỗi coordinator package-run.

use super::*;

#[test]
fn unsafe_execution_directory_is_typed_and_keeps_its_message() {
    let error = registry_package_execution_cwd_not_real();
    assert_eq!(
        error.to_string(),
        "package execution working directory must be a real directory"
    );
    assert!(matches!(
        error.downcast_ref::<RunRegistryPackageError>(),
        Some(RunRegistryPackageError::ExecutionCwdNotReal)
    ));
}
