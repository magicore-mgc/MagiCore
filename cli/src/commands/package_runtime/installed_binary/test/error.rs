//! Installed binary error contract regressions.
//! Test hồi quy contract lỗi executable đã cài.

use super::*;

#[test]
fn binary_failures_keep_their_typed_causes_and_messages() {
    let no_binary = registry_package_no_binary("mgc-tool", "@scope/pkg");
    assert_eq!(
        no_binary.to_string(),
        "package runner: no binary 'mgc-tool' found for package '@scope/pkg'. Is it an executable package?"
    );
    assert!(matches!(
        no_binary.downcast_ref::<InstalledBinaryError>(),
        Some(InstalledBinaryError::NoBinary { bin, package }) if bin == "mgc-tool" && package == "@scope/pkg"
    ));

    let outside = registry_package_binary_outside_project();
    assert_eq!(
        outside.to_string(),
        "package runner: executable resolves outside the verified temporary project"
    );
    assert!(matches!(
        outside.downcast_ref::<InstalledBinaryError>(),
        Some(InstalledBinaryError::BinaryOutsideProject)
    ));

    let mismatch = registry_package_bin_target_mismatch();
    assert_eq!(
        mismatch.to_string(),
        "package runner: executable does not match the requested package's declared target"
    );
    assert!(matches!(
        mismatch.downcast_ref::<InstalledBinaryError>(),
        Some(InstalledBinaryError::BinTargetMismatch)
    ));
}
