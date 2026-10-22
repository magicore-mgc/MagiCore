// Materialize a core-owned template — materialize template thuộc core.
// Core-owned template materialization.
use super::*;

impl Scaffolder {
    /// Materialize template đơn layer cho core non-web (chung cho game/iot/cloud/cicd/app/ai/lib).
    /// Context tối thiểu: project_name/slug/package, core, framework, features (Q13).
    pub(in crate::scaffold::processor) fn materialize_core_template(
        target: &Path,
        layer: &TemplateRoot,
        context: &CoreTemplateContext,
    ) -> Result<()> {
        let Some(manifest) = TemplateManifest::load(layer)? else {
            return Err(crate::scaffold::error::template_layer_missing_manifest(
                &layer.logical_rel(),
            ));
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
            let target_path = render_core_target_path(&file.target, context);
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
            let target_path = render_core_target_path(&file.target, context);
            let bytes = layer.read(&source_rel)?;
            match std::str::from_utf8(&bytes) {
                Ok(contents) => {
                    let rendered = context.render(
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
