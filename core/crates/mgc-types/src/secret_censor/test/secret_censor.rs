//! Verify the legacy literal censor contract.
//! Kiểm tra hợp đồng thay thế literal hiện tại.

use super::censor_secret;

#[test]
fn replaces_each_supported_lowercase_substring() {
    assert_eq!(
        censor_secret("token password secret key"),
        "**** **** **** ****"
    );
}

#[test]
fn preserves_case_and_replaces_matching_substrings_inside_words() {
    assert_eq!(
        censor_secret("Token=visible api_key=key"),
        "Token=visible api_****=****"
    );
}

#[test]
fn leaves_assignment_values_visible_after_replacing_the_key_name() {
    assert_eq!(censor_secret("token=visible"), "****=visible");
}
