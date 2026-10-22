//! Model identity validation tests.
//! Test kiểm tra định danh model.

use mgc_types::MgError;

#[test]
fn accepts_exact_org_name_and_commit() {
    super::super::check_model_id("org/model").expect("test operation must succeed");
    super::super::check_revision("0123456789abcdef0123456789abcdef01234567")
        .expect("test operation must succeed");
}

#[test]
fn rejects_invalid_model_and_mutable_revision_with_legacy_errors() {
    let model_error =
        super::super::check_model_id("../model").expect_err("test operation must fail");
    let model_error: MgError = model_error.into();
    assert!(matches!(
        model_error,
        MgError::Other(ref message)
            if message == "invalid model id '../model' — expected org/name (fail-closed)"
    ));

    let revision_error =
        super::super::check_revision("main").expect_err("test operation must fail");
    let revision_error: MgError = revision_error.into();
    let expected = concat!(
        "revision 'main' is not an exact 40-hex commit SHA — ",
        "branch/tag names are mutable refs and cannot lock (fail-closed)"
    );
    assert!(matches!(
        revision_error,
        MgError::Other(ref message) if message == expected
    ));
}
