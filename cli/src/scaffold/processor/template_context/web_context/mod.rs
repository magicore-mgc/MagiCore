// Web context owner wiring.
// Nối owner dựng context Web.

mod web_context;

pub(in crate::scaffold::processor) use web_context::WebTemplateContext;
