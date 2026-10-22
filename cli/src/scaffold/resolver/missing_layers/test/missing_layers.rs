use crate::scaffold::resolver::MissingLayersReport;

#[test]
fn empty_report_has_no_required_or_optional_missing_layers() {
    let report = MissingLayersReport::new();
    assert!(!report.has_required_missing());
    assert!(report.is_empty());
}

#[test]
fn required_only_report_tracks_required_layers() {
    let mut report = MissingLayersReport::new();
    report.add_required("web/frontend/nextjs".to_string());
    report.add_required("web/base/runtime".to_string());
    assert!(report.has_required_missing());
    assert!(!report.is_empty());
    assert_eq!(report.required.len(), 2);
    assert_eq!(report.optional.len(), 0);
}

#[test]
fn optional_only_report_does_not_report_required_failure() {
    let mut report = MissingLayersReport::new();
    report.add_optional("web/partial/extra".to_string());
    assert!(!report.has_required_missing());
    assert!(!report.is_empty());
    assert_eq!(report.required.len(), 0);
    assert_eq!(report.optional.len(), 1);
}

#[test]
fn mixed_report_formats_required_and_optional_guidance() {
    let mut report = MissingLayersReport::new();
    report.add_required("web/frontend/nextjs".to_string());
    report.add_optional("web/partial/extra".to_string());
    assert!(report.has_required_missing());
    assert!(!report.is_empty());
    let msg = report.format_error("web", "nextjs");
    assert!(msg.contains("Required scaffold layers missing"));
    assert!(msg.contains("web/frontend/nextjs"));
    assert!(msg.contains("mgc template fetch"));
    assert!(msg.contains("Optional layers not found"));
}

#[test]
fn report_format_includes_required_layer_count() {
    let mut report = MissingLayersReport::new();
    report.add_required("layer1".to_string());
    report.add_required("layer2".to_string());
    report.add_required("layer3".to_string());
    assert!(report.format_error("web", "nextjs").contains("(3 total)"));
}

#[test]
fn report_preserves_the_original_resolution_failure_cause() {
    let mut report = MissingLayersReport::new();
    report.add_required_failure(
        "web/shared/partials/base",
        "OS lock denied: operation not permitted",
    );
    let msg = report.format_error("web", "remix");
    assert!(msg.contains("web/shared/partials/base"));
    assert!(msg.contains("OS lock denied: operation not permitted"));
}
