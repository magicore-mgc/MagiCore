//! `mgc create-<game>` — tách từ core/game.rs (Phase 7 v5).

use anyhow::Result;

pub async fn run(
    framework: &str,
    project_name: &str,
    source: crate::commands::core::shared::source_flags::SourceFlags,
) -> Result<()> {
    // Phase 4: Parse scaffold spec sớm với typo detection
    use crate::scaffold::spec::{CoreKind, parse_scaffold_spec};
    let parsed_framework = if framework.is_empty() {
        None
    } else {
        Some(
            parse_scaffold_spec(CoreKind::Game, framework).map_err(|error| {
                crate::error::invalid_framework_spec("game", framework, &error.to_string())
            })?,
        )
    };
    let mut config = if let Some(spec) = &parsed_framework {
        crate::wizard::engine::ScaffoldConfig {
            core: "game".to_string(),
            sub_type: String::new(),
            frameworks: vec![spec.normalized_name.clone()],
            project_name: project_name.to_string(),
            features: Vec::new(),
            template_dir: std::path::PathBuf::new(),

            ..Default::default()
        }
    } else {
        crate::wizard::game::GameWizard::run()
    };
    config.project_name = project_name.to_string();
    super::source::apply(&mut config, source);
    if let Some(fw) = config.frameworks.first() {
        // Phase 3: Handle typed result
        let request = parsed_framework
            .as_ref()
            .map(|spec| spec.template_request())
            .unwrap_or_else(|| fw.to_string());
        let explicit_ref = parsed_framework.as_ref().is_some_and(|spec| {
            !matches!(
                &spec.requested_ref,
                crate::scaffold::spec::ScaffoldRef::Default
            )
        });
        match crate::commands::template::ensure_layer(&format!("game/{request}")).await {
            Ok(status) if status.is_available() => {}
            Ok(_) => {
                mgc_ui::warning(&format!(
                    "Optional game layer 'game/{}' not found; the registry layer or a local project is required",
                    fw
                ));
            }
            Err(e) if explicit_ref || !e.is_availability_failure() => return Err(e.into()),
            Err(e) => {
                // No built-in generator exists: a registry layer failure is
                // only recoverable when a project with this framework already
                // lives on this machine.
                // Không tồn tại generator nội bộ: lỗi registry layer chỉ cứu
                // được khi máy đã có project cùng framework.
                if crate::scaffold::framework_registry::is_supported("game", fw) {
                    mgc_ui::warning(&format!(
                        "Registry layer unavailable ({e}) — will reuse a local '{}' project if one exists",
                        fw
                    ));
                } else {
                    anyhow::bail!(
                        "Unsupported game framework '{}' and registry layer unavailable: {}",
                        fw,
                        e
                    )
                }
            }
        }
    }
    super::scaffold_and_save_metadata(&config)?;
    mgc_ui::success("Game project created. Run `mgc add-game <pkg>` or `mgc install-game` next.");
    Ok(())
}

#[cfg(test)]
#[path = "test/game.rs"]
mod tests;
