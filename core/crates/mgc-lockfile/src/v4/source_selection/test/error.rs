//! Keep source-selection diagnostics stable.
//! Giữ ổn định diagnostic khi chọn source.

#[test]
fn selection_error_displays_remain_stable() {
    use super::*;
    assert_eq!(
        SourceSelectionError::AmbiguousSource {
            package: "x".to_string()
        }
        .to_string(),
        "ambiguous source for 'x': multiple sources claim it at equal specificity"
    );
    assert_eq!(
        SourceSelectionError::AmbiguousFallback {
            package: "x".to_string()
        }
        .to_string(),
        "ambiguous fallback for 'x': multiple catch-all sources share the lowest priority"
    );
    let public_override = SourceSelectionError::PublicOverridePrivate {
        package: "x".to_string(),
        source: "public".to_string(),
    };
    assert_eq!(
        public_override.to_string(),
        "public source 'public' would override a trusted claim for 'x'"
    );
}
