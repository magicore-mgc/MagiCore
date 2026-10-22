//! Verify project-context accessors reflect the loaded execution configuration.
//! Kiểm tra accessor của context phản ánh đúng cấu hình thực thi đã nạp.

#[cfg(feature = "web")]
#[test]
fn execution_summary_uses_the_project_execution_fields() {
    let project = tempfile::tempdir().expect("temporary project directory must be created");
    let adapter = crate::factory::create_adapter_for(
        project.path(),
        &mgc_types::Ecosystem::Web,
        None,
        None,
        &[],
    )
    .expect("web adapter should be constructible without registry access");
    let config = mgc_config::project::ProjectConfig::new("summary", "web");
    let expected = format!(
        "architecture={}, lane={}, compatibility={}, native_targets={}",
        config.execution.architecture,
        config.execution.lane,
        config.execution.compatibility_layer,
        if config.execution.native_targets.is_empty() {
            "none".to_string()
        } else {
            config.execution.native_targets.join(", ")
        },
    );
    let context = super::ProjectContext {
        root: project.path().to_path_buf(),
        config,
        adapter,
    };

    assert_eq!(context.root(), project.path());
    assert_eq!(context.execution(), &context.config.execution);
    assert_eq!(context.execution_summary(), expected);
}
