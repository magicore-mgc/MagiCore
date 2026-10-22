//! Candidate planning must match Web virtual-store names and reject linked entries.
//! Candidate plan phải khớp tên virtual-store Web và chặn entry là symlink.

use super::super::plan_candidates;
use mgc_types::PackageId;
use std::fs;

#[test]
fn scoped_package_uses_the_materializer_directory_and_counts_regular_payload() {
    let temp = tempfile::tempdir().unwrap();
    let root = fs::canonicalize(temp.path()).unwrap();
    let package = PackageId::parse("@scope/pkg@1.2.3").unwrap();
    let package_dir = root.join("@scope+pkg@1.2.3");
    fs::create_dir(&package_dir).unwrap();
    fs::write(package_dir.join("payload.bin"), b"payload").unwrap();

    let (plan, projected) = plan_candidates(vec![package], Some(&root)).unwrap();

    assert_eq!(projected, 7);
    assert_eq!(plan.len(), 1);
    assert_eq!(plan[0].directory.as_deref(), Some(package_dir.as_path()));
    assert_eq!(plan[0].logical_bytes, 7);
}

#[test]
fn missing_package_directory_has_zero_projected_bytes() {
    let temp = tempfile::tempdir().unwrap();
    let root = fs::canonicalize(temp.path()).unwrap();
    let package = PackageId::parse("react@18.2.0").unwrap();

    let (plan, projected) = plan_candidates(vec![package], Some(&root)).unwrap();

    assert_eq!(projected, 0);
    assert_eq!(plan.len(), 1);
    assert!(plan[0].directory.is_none());
    assert_eq!(plan[0].logical_bytes, 0);
}

#[cfg(any(unix, windows))]
#[test]
fn linked_package_directory_fails_closed() {
    let temp = tempfile::tempdir().unwrap();
    let root = fs::canonicalize(temp.path()).unwrap();
    let outside = temp.path().join("external");
    fs::create_dir(&outside).unwrap();
    fs::write(outside.join("payload.bin"), b"external").unwrap();
    let package = PackageId::parse("react@18.2.0").unwrap();
    super::symlink_support::symlink_directory(&outside, &root.join("react@18.2.0")).unwrap();

    let result = plan_candidates(vec![package], Some(&root));

    assert!(matches!(
        result,
        Err(super::super::error::CandidatePlanError::Filesystem(
            crate::commands::safe_filesystem::SafeFilesystemError::SymlinkedDirectory { .. }
        ))
    ));
    assert_eq!(fs::read(outside.join("payload.bin")).unwrap(), b"external");
}
