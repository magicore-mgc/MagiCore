// Core context owner wiring.
// Nối owner dựng context cho các core không thuộc Web.

mod core_context;

pub(in crate::scaffold::processor) use core_context::CoreTemplateContext;
