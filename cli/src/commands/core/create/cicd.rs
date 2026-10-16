//! `mgc create cicd` — wizard + scaffold theo framework (07 §4).

use anyhow::Result;

pub async fn run(framework: &str, project_name: &str) -> Result<()> {
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
        }
    } else {
        crate::wizard::cicd::CicdWizard::run()
    };
    config.project_name = project_name.to_string();
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
                    "Optional cicd layer 'cicd/{}' not found, using fallback",
                    fw
                ));
            }
            Err(e) if explicit_ref || !e.is_availability_failure() => return Err(e.into()),
            Err(e) => {
                // Built-in generator fallback — but ONLY for frameworks
                // this core's processor actually generates; anything else
                // keeps the honest layer-required error (no mislabeled scaffold).
                // (Fallback generator nội bộ — chỉ framework processor hỗ trợ.)
                if crate::scaffold::processors::cicd::CicdProcessor::supports(fw) {
                    mgc_ui::warning(&format!(
                        "Registry layer unavailable ({e}) — using built-in cicd generator",
                    ));
                } else {
                    anyhow::bail!("Required cicd template layer missing: {}", e)
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
