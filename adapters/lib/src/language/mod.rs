//! Lib language module wiring and stable API facade.
//! Wiring module ngôn ngữ Lib và facade API ổn định.

mod dependency_format;
mod detection;
mod dotnet_project;
mod manifest_classification;
mod manifest_path;
mod types;

pub use dependency_format::dependency_manifest_format;
pub use detection::detect_language;
pub(crate) use dotnet_project::find_csproj;
pub(crate) use manifest_classification::manifest_is_lib;
pub(crate) use manifest_path::is_regular_manifest;
pub use types::LibLanguage;
