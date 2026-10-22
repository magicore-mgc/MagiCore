//! Maps a parsed scaffold request to its artifact layer path.
//! Ánh xạ yêu cầu scaffold đã parse sang đường dẫn layer artifact.

use crate::scaffold::spec::{CoreKind, ScaffoldSpec};

/// Helper: convert spec to layer path
pub fn spec_to_layer_path(spec: &ScaffoldSpec) -> String {
    // Ví dụ: web/frontend/nextjs, ai/fastapi, app/flutter
    match spec.core {
        CoreKind::Web => format!("web/frontend/{}", spec.normalized_name),
        CoreKind::Ai => format!("ai/{}", spec.normalized_name),
        CoreKind::App => format!("app/{}", spec.normalized_name),
        CoreKind::Lib => format!("lib/{}", spec.normalized_name),
        CoreKind::Game => format!("game/{}", spec.normalized_name),
        CoreKind::Iot => format!("iot/{}", spec.normalized_name),
        CoreKind::Cicd => format!("cicd/{}", spec.normalized_name),
        CoreKind::Cloud => format!("cloud/{}", spec.normalized_name),
        CoreKind::Hardware => format!("hardware/{}", spec.normalized_name),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
