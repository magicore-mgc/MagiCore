// template_manifest owner wiring — nối owner template_manifest.
// Keep the implementation and re-export boundary small.
use super::*;
#[path = "template_manifest.rs"]
mod implementation;
pub(in crate::scaffold::processor) use implementation::TemplateManifest;
pub(crate) use implementation::validate_template_manifest;
#[cfg(test)]
pub(in crate::scaffold::processor) use implementation::{
    MAX_TEMPLATE_MANIFEST_BYTES, TemplateFile,
};
