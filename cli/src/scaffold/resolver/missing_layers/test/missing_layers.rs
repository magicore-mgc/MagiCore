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
    assert!(msg.contains("--from-local"));
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

#[test]
fn required_layer_guidance_never_names_a_nonexistent_subcommand() {
    // `mgc template` is not a subcommand; telling the user to run it sent them
    // to a clap error. The remedy must be something they can actually type.
    // (`mgc template` không phải subcommand; bảo user chạy nó chỉ dẫn tới lỗi
    // clap. Cách xử lý phải là thứ gõ được thật.)
    let mut report = MissingLayersReport::new();
    report.add_required("game/engine/bevy".to_string());

    let msg = report.format_error("game", "bevy");

    assert!(
        !crate::scaffold::resolver::remedy::names_nonexistent_subcommand(&msg),
        "guidance must not name a subcommand that does not exist: {msg}"
    );
    assert!(msg.contains("--from-local <path>"), "{msg}");
    assert!(msg.contains("official initializer"), "{msg}");
}

#[test]
fn generator_backed_cores_are_pointed_at_the_offline_generator() {
    // bevy/godot/iot/cloud scaffold offline today, so the honest first remedy
    // for a layer miss on those cores is the generator itself.
    // (bevy/godot/iot/cloud hiện scaffold offline, nên cách xử lý đầu tiên trung
    // thực khi thiếu layer ở các core đó là chính generator.)
    for core in crate::scaffold::resolver::OFFLINE_GENERATOR_CORES {
        let mut report = MissingLayersReport::new();
        report.add_required(format!("{core}/layer"));

        let msg = report.format_error(core, "example");

        assert!(
            msg.contains(&format!("mgc create-{core}")),
            "core '{core}' must mention its offline generator: {msg}"
        );
        assert!(
            !crate::scaffold::resolver::remedy::names_nonexistent_subcommand(&msg),
            "core '{core}' guidance must not name a nonexistent subcommand: {msg}"
        );
    }
}

#[test]
fn registry_guidance_names_the_configuration_that_is_actually_read() {
    let mut report = MissingLayersReport::new();
    report.add_required("web/frontend/nextjs".to_string());

    let msg = report.format_error("web", "nextjs");

    assert!(msg.contains("MGC_NPM_REGISTRY"), "{msg}");
    assert!(!msg.contains("mgc config set registry.scaffold"), "{msg}");
}
