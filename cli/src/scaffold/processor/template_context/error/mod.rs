// Template-context error owner wiring.
// Nối owner lỗi cục bộ cho template context.

mod error;

pub(in crate::scaffold::processor::template_context) use error::TemplateContextError;
