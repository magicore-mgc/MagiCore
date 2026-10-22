//! Inspection error classification and source-chain tests.
//! Test phân loại lỗi inspection và giữ chuỗi nguồn.

use super::*;
use std::error::Error;

#[test]
fn member_inspection_error_keeps_its_source() {
    let error = InspectionError::MemberInspection(anyhow::Error::msg("snapshot failed"));
    assert_eq!(error.to_string(), "failed to inspect Windows Job members");
    assert_eq!(
        error.source().map(ToString::to_string).as_deref(),
        Some("snapshot failed")
    );
}

#[test]
fn command_validation_error_identifies_the_job_member_and_keeps_source() {
    let error = InspectionError::CommandLineValidation {
        process_id: 41,
        image_name: "tool.exe".to_owned(),
        source: anyhow::Error::msg("missing command line"),
    };
    assert!(error.to_string().contains("PID 41 (tool.exe)"));
    assert_eq!(
        error.source().map(ToString::to_string).as_deref(),
        Some("missing command line")
    );
}
