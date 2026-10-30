//! `mgc create-hardware <optimizer|bench>` — scaffold thẳng vào project. Phase 7 v5.

use anyhow::Result;

pub async fn run(
    framework: &str,
    project_name: &str,
    source: crate::commands::core::shared::source_flags::SourceFlags,
) -> Result<()> {
    // A framework argument (or resolved automation name) makes the run
    // non-interactive: skip the wizard so CI never blocks on a prompt.
    // Có tham số framework (hoặc automation đã phân giải) thì chạy không tương
    // tác: bỏ wizard để CI không bị chặn ở prompt.
    let mut config = if framework.is_empty() {
        crate::wizard::hardware::HardwareWizard::run()
    } else {
        crate::wizard::engine::ScaffoldConfig {
            core: "hardware".to_string(),
            sub_type: String::new(),
            frameworks: vec![framework.to_string()],
            project_name: project_name.to_string(),
            features: Vec::new(),
            template_dir: std::path::PathBuf::new(),

            ..Default::default()
        }
    };
    config.project_name = project_name.to_string();
    // Offline source selection is MagiCore-owned config, never framework source.
    // Chọn nguồn offline thuộc config của MagiCore, không phải source framework.
    config.from_local = source.from_local;
    config.no_local = source.no_local;
    if !framework.is_empty() {
        config.frameworks = vec![framework.to_string()];
    }
    match config.frameworks.first().map(|s| s.as_str()) {
        Some("optimizer") | Some("bench") => {}
        Some(other) => return Err(crate::error::unknown_hardware_framework(other)),
        None => return Err(crate::error::no_hardware_framework()),
    }
    super::scaffold_and_save_metadata(&config)?;
    mgc_ui::success(&format!(
        "Hardware '{}' scaffolded at '{project_name}'. Run `mgc add-hardware bench` or `mgc bench` to run benchmarks.",
        config.frameworks.first().map(|s| s.as_str()).unwrap_or("")
    ));
    Ok(())
}
