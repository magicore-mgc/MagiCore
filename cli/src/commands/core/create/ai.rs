//! `mgc create-<ai>` — tách từ core/ai.rs (Phase 7 v5).

use anyhow::Result;

pub async fn run(
    framework: &str,
    project_name: &str,
    container: super::super::shared::container_flags::ContainerFlags,
    source: crate::commands::core::shared::source_flags::SourceFlags,
) -> Result<()> {
    if container.k8s {
        return Err(crate::error::k8s_profile_unsupported("ai"));
    }
    // Phase 4: Parse scaffold spec sớm với typo detection
    use crate::scaffold::spec::{CoreKind, parse_scaffold_spec};
    let parsed_framework = if !framework.is_empty() {
        Some(parse_scaffold_spec(CoreKind::Ai, framework).map_err(|e| {
            anyhow::anyhow!("Invalid AI framework specification '{}': {}", framework, e)
        })?)
    } else {
        None
    };

    let mut config = if let Some(spec) = &parsed_framework {
        crate::wizard::engine::ScaffoldConfig {
            project_name: project_name.to_string(),
            frameworks: vec![spec.normalized_name.clone()],
            core: "ai".to_string(),
            sub_type: String::new(),
            features: Vec::new(),
            template_dir: std::path::PathBuf::new(),

            ..Default::default()
        }
    } else {
        crate::wizard::ai::AiWizard::run()
    };
    config.project_name = project_name.to_string();
    super::source::apply(&mut config, source);
    if container.docker && !config.features.iter().any(|feature| feature == "docker") {
        config.features.push("docker".to_string());
    }
    // The framework's own scaffolder is the first source, exactly as the game,
    // iot, clo and cicd cores already do. Without it an AI project fell through
    // to a registry layer that does not exist yet and produced an empty
    // directory.
    // (Scaffolder của chính framework là nguồn đầu tiên, giống các core game/
    // iot/clo/cicd. Thiếu bước này, project AI rơi xuống registry layer chưa có
    // và chỉ tạo thư mục rỗng.)
    if config.from_local.is_none() {
        let cache_framework = parsed_framework
            .as_ref()
            .map(|spec| spec.normalized_name.as_str())
            .unwrap_or(framework);
        crate::commands::template::ensure_cache_healthy(&format!("ai/{cache_framework}"))?;
    }
    if config.from_local.is_none()
        && let Some(framework) = parsed_framework
            .as_ref()
            .map(|spec| spec.normalized_name.as_str())
        && super::core_initializer::try_run("ai", framework, std::path::Path::new(project_name))?
    {
        mgc_ui::success(&format!("Created ai project: {project_name}"));
        return Ok(());
    }
    // `--from-local` names the user's own source and overrides every remote
    // layer, exactly like the web create route. Without this guard a core whose
    // registry layer is unpublished could never be scaffolded from a local
    // project even though the user pointed at one.
    // (`--from-local` chỉ định nguồn của user và thắng mọi layer từ xa, giống
    // route create của web. Thiếu chặn này, core chưa publish layer sẽ không
    // bao giờ scaffold được từ project local dù user đã chỉ định.)
    if config.from_local.is_none()
        && let Some(fw) = config.frameworks.first()
    {
        // Phase 3: Handle typed result, không bỏ qua
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
        match crate::commands::template::ensure_layer(&format!("ai/{request}")).await {
            Ok(status) if status.is_available() => {
                // Layer OK - proceed
            }
            Err(e) if explicit_ref || !e.is_availability_failure() => return Err(e.into()),
            // Fail closed: continuing after a warning left an empty project and
            // `mgc.toml` metadata for it, so the next command failed on a
            // directory with no sources.
            // (Fail-closed: tiếp tục sau cảnh báo để lại project rỗng; lệnh sau
            // hỏng trên thư mục không có source.)
            Ok(_) | Err(_) => {
                return Err(crate::scaffold::error::scaffold_source_unavailable(
                    "ai", fw,
                ));
            }
        }
    }
    super::scaffold_and_save_metadata(&config)?;

    mgc_ui::success(
        "AI project created. Pull a model with `mgc model pull hf://...` or run `mgc dev`.",
    );
    Ok(())
}

#[cfg(test)]
#[path = "test/ai.rs"]
mod tests;
