// Materialize validated Web template layers — materialize Web template layer đã kiểm tra.
// Web template materialization.
use super::*;

impl Scaffolder {
    pub(in crate::scaffold::processor) fn materialize_web_templates(
        target: &Path,
        config: &ScaffoldConfig,
        layers: &[TemplateRoot],
    ) -> Result<()> {
        let context = WebTemplateContext::new(config, layers);
        for layer in layers {
            Self::materialize_template_layer(target, layer, &context)?;
        }
        Ok(())
    }
    pub(in crate::scaffold::processor) fn materialize_template_layer(
        target: &Path,
        layer: &TemplateRoot,
        context: &WebTemplateContext,
    ) -> Result<()> {
        let Some(manifest) = TemplateManifest::load(layer)? else {
            return Ok(());
        };
        let active_features: HashSet<&str> = config_feature_set(&context.features);
        let active_files = manifest
            .files
            .iter()
            .filter(|file| !file.is_cli_generated_container_asset())
            .filter(|file| file.is_enabled(&active_features))
            .collect::<Vec<_>>();

        let mut seen_targets = HashSet::new();
        for file in &active_files {
            let target_path = render_target_path(&file.target, context);
            checked_template_path(target, &target_path)?;
            checked_template_path(layer.path(), &format!("sources/{}", file.source))?;
            if !seen_targets.insert(target_path.clone()) {
                return Err(crate::scaffold::error::duplicate_template_target(
                    &file.target,
                    &layer.logical_rel(),
                ));
            }
        }

        for file in active_files {
            let source_rel = format!("sources/{}", file.source);
            if !layer.exists(&source_rel) {
                return Err(crate::scaffold::error::template_source_missing(
                    &file.source,
                    &layer.logical_rel(),
                ));
            }

            let target_path = render_target_path(&file.target, context);
            let bytes = layer.read(&source_rel)?;
            match std::str::from_utf8(&bytes) {
                Ok(contents) => {
                    let rendered = context.render_with_contract(
                        contents,
                        &file.required_context,
                        &layer.label(&source_rel),
                    )?;
                    Self::write_file(&target.join(&target_path), &rendered)?;
                }
                Err(_) => {
                    if !file.required_context.is_empty() {
                        return Err(crate::scaffold::error::binary_source_with_context(
                            &layer.label(&source_rel),
                        ));
                    }
                    Self::write_bytes(&target.join(&target_path), &bytes)?;
                }
            }
        }

        Ok(())
    }
}
