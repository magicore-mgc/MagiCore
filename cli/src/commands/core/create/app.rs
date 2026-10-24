//! `mgc create app` — wizard + scaffold theo framework (07 §4).

use anyhow::Result;

pub async fn run(
    framework: &str,
    project_name: &str,
    container: super::super::shared::container_flags::ContainerFlags,
    source: crate::commands::core::shared::source_flags::SourceFlags,
) -> Result<()> {
    if container.k8s {
        return Err(crate::error::k8s_profile_unsupported("app"));
    }
    // Phase 4: Parse scaffold spec sớm với typo detection
    use crate::scaffold::spec::{CoreKind, parse_scaffold_spec};
    let parsed_framework = if !framework.is_empty() {
        Some(parse_scaffold_spec(CoreKind::App, framework).map_err(|e| {
            anyhow::anyhow!("Invalid app framework specification '{}': {}", framework, e)
        })?)
    } else {
        None
    };

    let mut config = if let Some(spec) = &parsed_framework {
        crate::wizard::engine::ScaffoldConfig {
            project_name: project_name.to_string(),
            frameworks: vec![spec.normalized_name.clone()],
            core: "app".to_string(),
            sub_type: String::new(),
            features: Vec::new(),
            template_dir: std::path::PathBuf::new(),

            ..Default::default()
        }
    } else {
        crate::wizard::app::AppWizard::run()
    };
    config.project_name = project_name.to_string();
    super::source::apply(&mut config, source);
    if container.docker && !config.features.iter().any(|feature| feature == "docker") {
        config.features.push("docker".to_string());
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
        match crate::commands::template::ensure_layer(&format!("app/{request}")).await {
            Ok(status) if status.is_available() => {}
            Ok(_) => {
                mgc_ui::warning(&format!(
                    "Optional app layer 'app/{}' not found; the registry layer or a local project is required",
                    fw
                ));
            }
            Err(e) if explicit_ref || !e.is_availability_failure() => return Err(e.into()),
            Err(e) => mgc_ui::warning(&format!(
                "Optional app layer 'app/{}' is unavailable; the registry layer or a local project is required: {}",
                fw, e
            )),
        }
    }
    super::scaffold_and_save_metadata(&config)?;
    mgc_ui::success("App project created. Run `mgc install` or `mgc dev` next.");
    Ok(())
}

#[cfg(test)]
#[path = "test/app.rs"]
mod tests;
