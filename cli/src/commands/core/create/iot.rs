//! `mgc create iot` — wizard + scaffold theo framework (07 §4).

use anyhow::Result;

pub async fn run(framework: &str, project_name: &str) -> Result<()> {
    // Phase 4: Parse scaffold spec sớm với typo detection
    use crate::scaffold::spec::{CoreKind, parse_scaffold_spec};
    let parsed_framework = if framework.is_empty() {
        None
    } else {
        Some(
            parse_scaffold_spec(CoreKind::Iot, framework).map_err(|error| {
                crate::error::invalid_framework_spec("iot", framework, &error.to_string())
            })?,
        )
    };
    let config = if let Some(spec) = &parsed_framework {
        // Keep board IDs such as `nrf52dk_nrf52832` byte-exact: the generic
        // scaffold parser normalizes underscores for registry template names.
        // Giữ nguyên ID board có dấu gạch dưới; parser chung chuẩn hóa tên layer.
        let requested = spec.name.to_ascii_lowercase();
        let canonical =
            crate::scaffold::processors::iot::IotProcessor::framework_for_board(&requested)
                .unwrap_or(&requested)
                .to_string();
        if !crate::scaffold::processors::iot::IotProcessor::supports(&canonical) {
            anyhow::bail!("Unsupported IoT framework '{}'.", requested);
        }
        let board =
            if crate::scaffold::processors::iot::IotProcessor::framework_for_board(&requested)
                .is_some()
            {
                Some(requested.clone())
            } else {
                crate::scaffold::processors::iot::IotProcessor::default_board(&canonical)
                    .map(str::to_string)
            }
            .ok_or_else(|| anyhow::anyhow!("No default board is registered for '{canonical}'."))?;
        crate::wizard::engine::ScaffoldConfig {
            core: "iot".to_string(),
            sub_type: String::new(),
            frameworks: vec![canonical],
            project_name: project_name.to_string(),
            features: vec![board],
            template_dir: std::path::PathBuf::new(),
        }
    } else {
        crate::wizard::iot::IotWizard::run()
    };

    let mut config = config;
    config.project_name = project_name.to_string();
    if let Some(fw) = config.frameworks.last() {
        // Phase 3: Handle typed result
        let request = parsed_framework
            .as_ref()
            .map(|spec| {
                let mut canonical = spec.clone();
                canonical.normalized_name = fw.clone();
                canonical.template_request()
            })
            .unwrap_or_else(|| fw.to_string());
        let explicit_ref = parsed_framework.as_ref().is_some_and(|spec| {
            !matches!(
                &spec.requested_ref,
                crate::scaffold::spec::ScaffoldRef::Default
            )
        });
        match crate::commands::template::ensure_layer(&format!("iot/{request}")).await {
            Ok(status) if status.is_available() => {}
            Ok(_) => {
                mgc_ui::warning(&format!(
                    "Optional iot layer 'iot/{}' not found, using fallback",
                    fw
                ));
            }
            Err(e) if explicit_ref || !e.is_availability_failure() => return Err(e.into()),
            Err(e) => {
                // Built-in generator fallback — but ONLY for frameworks
                // this core's processor actually generates; anything else
                // keeps the honest layer-required error (no mislabeled scaffold).
                // (Fallback generator nội bộ — chỉ framework processor hỗ trợ.)
                if crate::scaffold::processors::iot::IotProcessor::supports(fw) {
                    mgc_ui::warning(&format!(
                        "Registry layer unavailable ({e}) — using built-in iot generator",
                    ));
                } else {
                    anyhow::bail!("Required iot template layer missing: {}", e)
                }
            }
        }
    }
    super::scaffold_and_save_metadata(&config)?;
    mgc_ui::success("IoT project created. Run `mgc add-iot <pkg>` or `mgc install-iot` next.");
    Ok(())
}

#[cfg(test)]
#[path = "test/iot.rs"]
mod tests;
