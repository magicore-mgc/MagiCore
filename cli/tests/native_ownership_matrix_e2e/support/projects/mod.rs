//! Project fixture owner exports. / Export fixture dự án.

mod ai_app;
mod game_iot;
mod library;
mod web;

pub(crate) use ai_app::{
    ai_project, flutter_project, flutter_with_meta, kotlin_with_dep, swift_with_dep,
};
pub(crate) use game_iot::{bevy_project, bevy_with_dep, esp32_project, esp32_with_dep};
pub(crate) use library::{
    dotnet_manifest, go_manifest, java_manifest, lib_project, python_manifest, rust_manifest, write,
};
pub(crate) use web::{WebBackendCase, web_backend_nonempty_add_install, web_backend_project};
