//! Behavior tests for `ScaffoldSpec`.
//! Test hành vi cho `ScaffoldSpec`.

use super::super::ScaffoldSpec;
use crate::scaffold::spec::ScaffoldRef;

fn request(reference: ScaffoldRef) -> ScaffoldSpec {
    ScaffoldSpec {
        core: crate::scaffold::spec::CoreKind::Web,
        name: "React_Native".to_string(),
        normalized_name: "react-native".to_string(),
        requested_ref: reference,
    }
}

#[test]
fn template_request_preserves_default_and_explicit_reference_forms() {
    assert_eq!(
        request(ScaffoldRef::Default).template_request(),
        "react-native"
    );
    assert_eq!(
        request(ScaffoldRef::Version("18.3.0".to_string())).template_request(),
        "react-native@18.3.0"
    );
    assert_eq!(
        request(ScaffoldRef::DistTag("beta".to_string())).template_request(),
        "react-native@beta"
    );
    assert_eq!(
        request(ScaffoldRef::Range("^1.0".to_string())).template_request(),
        "react-native@^1.0"
    );
}
