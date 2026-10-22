//! Detects supported backend manifests for Web full-stack mode.
//! Nhận diện manifest backend được hỗ trợ cho mode full-stack Web.

use std::path::Path;

pub(in crate::commands::core::web::implementation::workspace_layout) fn has_backend_manifest(
    dir: &Path,
) -> bool {
    dir.join("package.json").exists()
        || dir.join("go.mod").exists()
        || dir.join("Cargo.toml").exists()
        || dir.join("manage.py").exists()
        || dir.join("main.py").exists()
        || dir.join("src/main.py").exists()
        || dir.join("pom.xml").exists()
        || dir.join("artisan").exists()
        || dir.join("composer.json").exists()
}
