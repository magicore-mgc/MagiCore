//! `mgc create iot` — wizard + scaffold theo framework (07 §4).

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
        let canonical = crate::scaffold::framework_registry::iot_framework_for_board(&requested)
            .unwrap_or(&requested)
            .to_string();
        if !crate::scaffold::framework_registry::is_supported("iot", &canonical) {
            anyhow::bail!("Unsupported IoT framework '{}'.", requested);
        }
        let board = if crate::scaffold::framework_registry::iot_framework_for_board(&requested)
            .is_some()
        {
            Some(requested.clone())
        } else {
            crate::scaffold::framework_registry::iot_default_board(&canonical).map(str::to_string)
        }
        .ok_or_else(|| anyhow::anyhow!("No default board is registered for '{canonical}'."))?;
        crate::wizard::engine::ScaffoldConfig {
            core: "iot".to_string(),
            sub_type: String::new(),
            frameworks: vec![canonical],
            project_name: project_name.to_string(),
            features: vec![board],
            template_dir: std::path::PathBuf::new(),

            ..Default::default()
        }
    } else {
        crate::wizard::iot::IotWizard::run()
    };

    let mut config = config;
    config.project_name = project_name.to_string();
    super::source::apply(&mut config, source);
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
                    "Optional iot layer 'iot/{}' not found; the registry layer or a local project is required",
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
                if crate::scaffold::framework_registry::is_supported("iot", fw) {
                    mgc_ui::warning(&format!(
                        "Registry layer unavailable ({e}) — will reuse a local '{}' project if one exists",
                        fw
                    ));
                } else {
                    anyhow::bail!(
                        "Unsupported iot framework '{}' and registry layer unavailable: {}",
                        fw,
                        e
                    )
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
