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

    // Built-in local generator first (offline): the game adapter ships real file
    // writers for bevy/godot/unity/unreal. Generating here keeps an offline
    // machine from dead-ending at a registry 404 and never spawns a host engine
    // CLI — the framework's own program is only used below for engines this core
    // has no generator for.
    // (Local generator có sẵn chạy trước, offline: adapter game có generator file
    // thật cho bevy/godot/unity/unreal. Sinh ở đây để máy offline không chết ở
    // registry 404 và không spawn CLI engine host — chương trình của framework
    // chỉ dùng bên dưới cho engine core này không có generator.)
    // An explicit `--from-local` owns the source choice: never overwrite it.
    // (`--from-local` do người dùng chọn nguồn: không ghi đè.)
    if config.from_local.is_none()
        && let Some(fw) = config.frameworks.first()
    {
        let target = std::path::Path::new(project_name);
        if super::local_generator::try_local_scaffold(
            &super::local_generator::LocalScaffoldRequest {
                core: "game",
                framework: fw,
                target,
                board: None,
            },
        )
        .await?
        {
            // The adapter already created the files, so write only MagiCore's
            // metadata (mgc.toml + .mgc.core) — re-scaffolding would collide
            // with the directory the generator just created.
            // (Adapter đã tạo file, nên chỉ ghi metadata MagiCore; scaffold lại
            // sẽ đụng thư mục generator vừa tạo.)
            super::save_scaffold_metadata(target, &config)?;
            mgc_ui::success(&format!(
                "Created game project '{project_name}' with the built-in '{fw}' generator (offline)."
            ));
            return Ok(());
        }
    }

    // The framework's own program is the next source, for engines with no
    // built-in generator. Running it early keeps a stale local project from
    // masking a broken initializer.
    // Chương trình của chính framework là nguồn kế tiếp, cho engine không có
    // generator. Chạy sớm để project local cũ không che initializer hỏng.
    if config.from_local.is_none()
        && let Some(framework) = config.frameworks.first()
    {
        let target = std::path::Path::new(project_name);
        if super::core_initializer::try_run("game", framework, target)? {
            mgc_ui::success(&format!("Created game project: {project_name}"));
            return Ok(());
        }
    }
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
