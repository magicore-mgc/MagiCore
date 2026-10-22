//! Metafile document shape tests.
//! Test shape document metafile.

use super::super::super::document::validate_document;
use super::super::super::error::MetafileInputsError;

#[test]
fn requires_exactly_one_non_empty_inputs_map() {
    let accepts_all = |_: &str| -> Result<(), MetafileInputsError> { Ok(()) };

    assert!(validate_document(r#"{"inputs":{"inside.js":{}},"outputs":{}}"#, accepts_all).is_ok());
    assert!(matches!(
        validate_document(r#"{"outputs":{}}"#, accepts_all),
        Err(MetafileInputsError::InvalidMetadata)
    ));
    assert!(matches!(
        validate_document(r#"{"inputs":{}}"#, accepts_all),
        Err(MetafileInputsError::InvalidMetadata)
    ));
    assert!(matches!(
        validate_document(r#"{"inputs":{"inside.js":[]}}"#, accepts_all),
        Err(MetafileInputsError::InvalidMetadata)
    ));
    assert!(matches!(
        validate_document(
            r#"{"inputs":{"one.js":{}},"inputs":{"two.js":{}}}"#,
            accepts_all
        ),
        Err(MetafileInputsError::InvalidMetadata)
    ));
}
