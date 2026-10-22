//! App language owner wiring and stable exports.
//! Wiring owner ngôn ngữ App và giữ export ổn định.

mod app_language;
mod explicit_language;
mod framework_markers;
mod language;
mod manifest_is_app;
mod maui_project_manifest;
mod regular_manifest;

pub use app_language::AppLanguage;
pub use language::detect_language;
pub(crate) use manifest_is_app::manifest_is_app;
pub(crate) use maui_project_manifest::maui_project_manifest;
pub(crate) use regular_manifest::is_regular_manifest;
