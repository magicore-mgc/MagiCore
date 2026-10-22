//! Dependency bundle diagnostic formatting test.
//! Test format diagnostic bundle dependency.

use super::BundleError;

#[test]
fn joins_esbuild_messages_in_the_existing_order() {
    let error = BundleError::Esbuild(vec!["first error".to_string(), "second error".to_string()]);

    assert_eq!(error.to_string(), "first error; second error");
}

#[test]
fn reports_metafile_rejections_without_including_an_input_path() {
    let error = BundleError::MetafileInputs(
        super::super::super::build::MetafileInputsError::OutsideAllowedRoots,
    );

    assert_eq!(
        error.to_string(),
        "dependency bundle input rejected: esbuild included an input outside installed dependency roots"
    );
}
