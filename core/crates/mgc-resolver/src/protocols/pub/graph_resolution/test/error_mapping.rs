//! Local graph errors retain their established shared categories.
//! Test lỗi graph giữ nguyên nhóm MgError hiện hành.

use super::super::GraphResolutionError;
use mgc_types::MgError;

#[test]
fn missing_metadata_remains_integrity_failure() {
    let error = GraphResolutionError::MissingCachedMetadata("sample".to_string()).into_mg_error();
    assert!(
        matches!(error, MgError::Integrity(message) if message == "missing cached pub.dev metadata for 'sample'")
    );
}

#[test]
fn unstable_selection_remains_dependency_conflict() {
    let error = GraphResolutionError::OscillatingSelection.into_mg_error();
    assert!(
        matches!(error, MgError::DependencyConflict(message) if message == "pub.dev dependency selection oscillates across graph passes; refusing to emit an unstable lock graph")
    );
}
