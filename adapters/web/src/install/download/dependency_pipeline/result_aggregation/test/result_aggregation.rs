//! Tests for dependency pipeline result aggregation.
//! Test gom kết quả pipeline dependency.

use std::path::PathBuf;

use mgc_types::PackageId;

use super::{PipelineTaskOutput, aggregate_pipeline_results};

#[test]
fn aggregates_bytes_roots_and_persist_handles() {
    let first = PackageId::parse("react@19.0.0").expect("valid package id");
    let second = PackageId::parse("vite@7.0.0").expect("valid package id");
    let first_root = PathBuf::from("store/react");
    let second_root = PathBuf::from("store/vite");
    let outputs: Vec<Result<PipelineTaskOutput, mgc_types::MgError>> = vec![
        Ok((17, first.clone(), first_root.clone(), None)),
        Ok((25, second.clone(), second_root.clone(), None)),
    ];

    let (bytes, roots, handles) =
        aggregate_pipeline_results(outputs).expect("successful outputs aggregate");

    assert_eq!(bytes, 42);
    assert_eq!(roots.get(&first), Some(&first_root));
    assert_eq!(roots.get(&second), Some(&second_root));
    assert!(handles.is_empty());
}
