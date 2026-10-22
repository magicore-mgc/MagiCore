//! Web runtime owner exports and nearest tests.
mod web;
pub(in crate::commands::optimizer) use web::detect_web_runtime;

#[cfg(test)]
#[path = "test/web.rs"]
mod tests;
