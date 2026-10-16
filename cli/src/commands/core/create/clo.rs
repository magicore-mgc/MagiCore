//! `mgc create-<clo>` — tách từ core/clo.rs (Phase 7 v5).

use anyhow::Result;

pub async fn run(
    framework: &str,
    project_name: &str,
    container: super::super::scaffold_flags::ContainerFlags,
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
        }
    } else {
        crate::wizard::cloud::CloudWizard::run()
    };
    config.project_name = project_name.to_string();
    if container.docker && !config.features.iter().any(|feature| feature == "docker") {
        config.features.push("docker".to_string());
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
        match crate::commands::template::ensure_layer(&format!("cloud/{request}")).await {
            Ok(status) if status.is_available() => {}
            Ok(_) => {
                mgc_ui::warning(&format!(
                    "Optional cloud layer 'cloud/{}' not found, using fallback",
                    fw
                ));
            }
            Err(e) if explicit_ref || !e.is_availability_failure() => return Err(e.into()),
            Err(e) => {
                // Built-in generator fallback — but ONLY for frameworks
                // this core's processor actually generates; anything else
                // keeps the honest layer-required error (no mislabeled scaffold).
                // (Fallback generator nội bộ — chỉ framework processor hỗ trợ.)
                if crate::scaffold::processors::clo::CloProcessor::supports(fw) {
                    mgc_ui::warning(&format!(
                        "Registry layer unavailable ({e}) — using built-in cloud generator",
                    ));
                } else {
                    anyhow::bail!("Required cloud template layer missing: {}", e)
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
