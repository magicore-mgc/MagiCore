#![allow(clippy::unwrap_used)]
//! JavaScript data-encoding tests for dependency responses.
//! Test encode dữ liệu JavaScript cho response dependency.

use super::super::{css_injector, missing_dependency_diagnostic};

#[test]
fn css_is_serialized_as_data_and_comment_is_request_independent() {
    let payload = css_injector("body{content:'${not_code}'}");

    assert!(payload.contains(r#"replaceSync("body{content:'${not_code}'}")"#));
    assert!(payload.contains("// [MgDevServer] Injected package CSS"));
    assert!(!payload.contains("Injected package CSS from"));
}

#[test]
fn failed_package_name_is_encoded_as_a_javascript_string() {
    let payload = missing_dependency_diagnostic("x');alert(1);//");

    assert_eq!(
        payload,
        r#"console.error('[MgDevServer] Failed to bundle dependency:', "x');alert(1);//");"#
    );
}
