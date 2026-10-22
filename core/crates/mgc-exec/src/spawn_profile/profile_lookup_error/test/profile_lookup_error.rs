use super::super::ProfileLookupError;

#[test]
fn distinguishes_absence_from_invalid_profile_data() {
    let missing = ProfileLookupError::NotFound;
    let invalid = ProfileLookupError::Invalid("bad".to_string());
    assert_ne!(missing, invalid);
    assert_eq!(format!("{missing:?}"), "NotFound");
}
