//! Validate a layer contract and keep its existing readiness error.
//! Xác thực contract layer và giữ nguyên lỗi readiness hiện có.

use crate::scaffold::processor::Scaffolder;
use crate::scaffold::template_root::TemplateRoot;
use anyhow::Result;

impl Scaffolder {
    pub(in crate::scaffold::processor) fn layer_has_contract(layer: &TemplateRoot) -> bool {
        layer.exists("template.toml") && layer.exists("sources")
    }

    pub(in crate::scaffold::processor) fn ensure_web_layer_ready(
        layer: &TemplateRoot,
        label: &str,
    ) -> Result<()> {
        if Self::layer_has_contract(layer) {
            return Ok(());
        }

        Err(crate::scaffold::error::scaffold_not_implemented(
            label,
            &layer.logical_rel(),
        ))
    }
}
