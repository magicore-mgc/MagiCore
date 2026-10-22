//! Root-exit error source and classification tests.
//! Test phân loại lỗi root-exit và giữ chuỗi nguồn.

use super::*;
use std::error::Error;

#[test]
fn root_exit_error_preserves_inspection_source() {
    let error = RootExitError::DescendantInspection(anyhow::Error::msg("member lookup failed"));
    assert_eq!(
        error.to_string(),
        "failed to inspect Windows Job descendants after root exit"
    );
    assert_eq!(
        error.source().map(ToString::to_string).as_deref(),
        Some("member lookup failed")
    );
}

#[test]
fn root_exit_error_preserves_cleanup_phase_and_source() {
    let error = RootExitError::Termination {
        phase: "root-exit cleanup",
        source: anyhow::Error::msg("native terminate failed"),
    };
    assert!(error.to_string().contains("root-exit cleanup"));
    assert_eq!(
        error.source().map(ToString::to_string).as_deref(),
        Some("native terminate failed")
    );
}
