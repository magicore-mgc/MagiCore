//! Audience matching and wire-shape tests — test so khớp và dạng JSON audience.
//!
use super::super::Audience;

#[test]
fn matches_single_and_multiple_audiences() {
    assert!(Audience::Single("registry".into()).contains("registry"));
    let multiple = Audience::Multiple(vec!["registry".into(), "sigstore".into()]);
    assert!(multiple.contains("sigstore"));
    assert!(!multiple.contains("other"));
}

#[test]
fn serializes_and_deserializes_untagged_string_or_array() {
    let single = Audience::Single("registry".into());
    let multiple = Audience::Multiple(vec!["registry".into(), "sigstore".into()]);
    assert_eq!(
        serde_json::to_string(&single).expect("test operation must succeed"),
        "\"registry\""
    );
    assert_eq!(
        serde_json::from_str::<Audience>("\"registry\"").expect("test operation must succeed"),
        single
    );
    assert_eq!(
        serde_json::to_string(&multiple).expect("test operation must succeed"),
        "[\"registry\",\"sigstore\"]"
    );
    assert_eq!(
        serde_json::from_str::<Audience>("[\"registry\",\"sigstore\"]")
            .expect("test operation must succeed"),
        multiple
    );
}
