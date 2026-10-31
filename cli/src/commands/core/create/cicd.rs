//! `mgc create cicd` — wizard + scaffold theo framework (07 §4).
//!
//! Source order: an explicit `--from-local` still wins (it is resolved later and
//! skips every branch), otherwise the built-in offline generator, then the
//! provider's own initializer, then the registry layer. The cicd core publishes
//! no registry layer, so before the built-in generator existed every
//! `create-cicd` dead-ended at "no scaffold source" even though `mgc
//! ci-generate` could already emit a real pipeline.
//! Thứ tự nguồn: `--from-local` tường minh vẫn thắng (nó được resolve sau và bỏ
//! qua mọi nhánh), nếu không thì generator offline có sẵn, rồi initializer của
//! provider, rồi registry layer. Core cicd chưa có registry layer, nên trước khi
//! có generator có sẵn mọi `create-cicd` đều chết ở "no scaffold source" dù
//! `mgc ci-generate` đã sinh được pipeline thật.

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
            parse_scaffold_spec(CoreKind::Cicd, framework).map_err(|error| {
                crate::error::invalid_framework_spec("cicd", framework, &error.to_string())
            })?,
        )
    };
    let mut config = if let Some(spec) = &parsed_framework {
        crate::wizard::engine::ScaffoldConfig {
            core: "cicd".to_string(),
            sub_type: String::new(),
            frameworks: vec![spec.normalized_name.clone()],
            project_name: project_name.to_string(),
            features: Vec::new(),
            template_dir: std::path::PathBuf::new(),

            ..Default::default()
        }
    } else {
        crate::wizard::cicd::CicdWizard::run()
    };
    config.project_name = project_name.to_string();
    super::source::apply(&mut config, source);

    // The built-in generator owns the offline path: it writes the provider's
    // pipeline through the SAME emitter `mgc ci-generate` uses, plus README and
    // .gitignore, with no network and no provider CLI. An explicit
    // `--from-local` still wins because the user named the source.
    // (Generator có sẵn sở hữu đường offline: ghi pipeline của provider qua
    // ĐÚNG emitter `mgc ci-generate` dùng, kèm README và .gitignore, không mạng,
    // không CLI provider. `--from-local` tường minh vẫn thắng.)
    if config.from_local.is_none()
        && let Some(fw) = config.frameworks.first()
    {
        let target = std::path::Path::new(project_name);
        if super::local_generator::try_local_scaffold(
            &super::local_generator::LocalScaffoldRequest {
                core: "cicd",
                framework: fw,
                target,
                board: None,
            },
        )
        .await?
        {
            // The generator already wrote the project files, so write only the
            // metadata MagiCore owns (mgc.toml with `[cicd] provider`, .mgc.core).
            // (Generator đã ghi file project, nên chỉ ghi metadata MagiCore sở
            // hữu: mgc.toml với `[cicd] provider`, .mgc.core.)
            super::save_scaffold_metadata(target, &config)?;
            mgc_ui::success(&format!(
                "Created cicd project '{project_name}' with the built-in '{fw}' generator (offline)."
            ));
            mgc_ui::success(
                "Deployment remains unavailable until a native provider engine is implemented.",
            );
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
        if super::core_initializer::try_run("cicd", framework, target)? {
            mgc_ui::success(&format!("Created cicd project: {project_name}"));
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
        match crate::commands::template::ensure_layer(&format!("cicd/{request}")).await {
            Ok(status) if status.is_available() => {}
            Ok(_) => {
                mgc_ui::warning(&format!(
                    "Optional cicd layer 'cicd/{}' not found; the registry layer or a local project is required",
                    fw
                ));
            }
            Err(e) if explicit_ref || !e.is_availability_failure() => return Err(e.into()),
            Err(e) => {
                // The built-in generator declined this provider, so a registry
                // layer failure is only recoverable when a project with this
                // framework already lives on this machine.
                // Generator có sẵn từ chối provider này, nên lỗi registry layer
                // chỉ cứu được khi máy đã có project cùng framework.
                if crate::scaffold::framework_registry::is_supported("cicd", fw) {
                    mgc_ui::warning(&format!(
                        "Registry layer unavailable ({e}) — will reuse a local '{}' project if one exists",
                        fw
                    ));
                } else {
                    anyhow::bail!(
                        "Unsupported cicd framework '{}' and registry layer unavailable: {}",
                        fw,
                        e
                    )
                }
            }
        }
    }
    super::scaffold_and_save_metadata(&config)?;
    mgc_ui::success(
        "CICD project created. `mgc deploy` remains unavailable until a native provider engine is implemented.",
    );
    Ok(())
}

#[cfg(test)]
#[path = "test/cicd.rs"]
mod tests;
