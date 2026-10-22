//! App runtime owner exports and nearest tests.
mod app;
pub(in crate::commands::optimizer) use app::detect_app_runtime;

#[cfg(test)]
#[path = "test/app.rs"]
mod tests;
