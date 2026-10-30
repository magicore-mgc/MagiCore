//! `mgc create-<clo>` — tách từ core/clo.rs (Phase 7 v5).

use anyhow::Result;

pub async fn run(
    framework: &str,
    project_name: &str,
    container: super::super::shared::container_flags::ContainerFlags,
    source: crate::commands::core::shared::source_flags::SourceFlags,
) -> Result<()> {
    if container.k8s {
        return Err(crate::error::k8s_profile_unsupported("clo"));
    }
    // Phase 4: Parse scaffold spec sớm với typo detection
    use crate::scaffold::spec::{CoreKind, parse_scaffold_spec};
    let parsed_framework = if !framework.is_empty() {
        Some(
            parse_scaffold_spec(CoreKind::Cloud, framework).map_err(|e| {
                crate::error::invalid_framework_spec("cloud", framework, &e.to_string())
            })?,
        )
    } else {
        None
    };

    let mut config = if let Some(spec) = &parsed_framework {
        crate::wizard::engine::ScaffoldConfig {
            core: "clo".to_string(),
            sub_type: String::new(),
            frameworks: vec![spec.normalized_name.clone()],
            project_name: project_name.to_string(),
            features: Vec::new(),
            template_dir: std::path::PathBuf::new(),

            ..Default::default()
        }
    } else {
        crate::wizard::cloud::CloudWizard::run()
    };
    config.project_name = project_name.to_string();
    super::source::apply(&mut config, source);

    // Docker is generated-project metadata, so record it before either the local
    // generator or the registry path writes metadata.
    // (Feature docker là metadata của project sinh ra, nên ghi nhận trước khi
    // local generator hoặc đường registry ghi metadata.)
    if container.docker && !config.features.iter().any(|feature| feature == "docker") {
        config.features.push("docker".to_string());
    }

    // Built-in local generator first (offline): the cloud adapter ships real file
    // writers for cdk/pulumi/terraform/cloudflare. Generating here keeps an
    // offline machine from dead-ending at a registry 404 and never spawns
    // terraform/cdk/pulumi — the framework's own program is only used below for
    // providers this core has no generator for.
    // (Local generator có sẵn chạy trước, offline: adapter cloud có generator file
    // thật cho cdk/pulumi/terraform/cloudflare. Sinh ở đây để máy offline không
    // chết ở registry 404 và không spawn terraform/cdk/pulumi — chương trình của
    // framework chỉ dùng bên dưới cho provider core này không có generator.)
    // An explicit `--from-local` owns the source choice: never overwrite it.
    // (`--from-local` do người dùng chọn nguồn: không ghi đè.)
    if config.from_local.is_none()
        && let Some(fw) = config.frameworks.first()
    {
        let target = std::path::Path::new(project_name);
        if super::local_generator::try_local_scaffold(
            &super::local_generator::LocalScaffoldRequest {
                core: "clo",
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
                "Created cloud project '{project_name}' with the built-in '{fw}' generator (offline)."
            ));
            return Ok(());
        }
    }

    // The framework's own program is the next source, for providers with no
    // built-in generator. Running it early keeps a stale local project from
    // masking a broken initializer.
    // Chương trình của chính framework là nguồn kế tiếp, cho provider không có
    // generator. Chạy sớm để project local cũ không che initializer hỏng.
    if config.from_local.is_none()
        && let Some(framework) = config.frameworks.first()
    {
        let target = std::path::Path::new(project_name);
        if super::core_initializer::try_run("clo", framework, target)? {
            mgc_ui::success(&format!("Created clo project: {project_name}"));
            return Ok(());
        }
    }
    if let Some(fw) = config.frameworks.first() {
        // Phase 3: Handle typed result
        // Layer namespace is `cloud/` (CoreKind::from_str_core knows
        // "cloud", not the CLI alias "clo" — resolver maps Cloud →
        // "cloud/..."; using "clo/" here fails every create-clo).
        // (Namespace layer là `cloud/`, không phải alias CLI `clo`.)
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
        let namespace = crate::scaffold::framework_registry::template_registry_namespace("clo");
        match crate::commands::template::ensure_layer(&format!("{namespace}/{request}")).await {
            Ok(status) if status.is_available() => {}
            Ok(_) => {
                mgc_ui::warning(&format!(
                    "Optional cloud layer 'cloud/{}' not found; the registry layer or a local project is required",
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
                if crate::scaffold::framework_registry::is_supported("clo", fw) {
                    mgc_ui::warning(&format!(
                        "Registry layer unavailable ({e}) — will reuse a local '{}' project if one exists",
                        fw
                    ));
                } else {
                    anyhow::bail!(
                        "Unsupported clo framework '{}' and registry layer unavailable: {}",
                        fw,
                        e
                    )
                }
            }
        }
    }
    super::scaffold_and_save_metadata(&config)?;
    mgc_ui::success("Cloud project created. Run `mgc add-clo <pkg>` or `mgc install-clo` next.");
    Ok(())
}

#[cfg(test)]
#[path = "test/clo.rs"]
mod tests;
