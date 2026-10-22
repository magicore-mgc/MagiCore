// Template-context module wiring and processor-facing exports.
// Nối các owner context và giữ export tương thích cho processor.

mod core_context;
mod error;
mod rendering;
mod web_context;

pub(in crate::scaffold::processor) use core_context::CoreTemplateContext;
pub(in crate::scaffold::processor) use rendering::{render_core_target_path, render_target_path};
pub(in crate::scaffold::processor) use web_context::WebTemplateContext;
