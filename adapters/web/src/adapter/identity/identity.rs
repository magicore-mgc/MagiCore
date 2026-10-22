//! `identity.rs` — Web identity and project detection / định danh và nhận diện project web.
use super::super::WebAdapter;
use async_trait::async_trait;
use mgc_adapter_base::BaseAdapter;
use mgc_types::capabilities::{CoreIdent, ProjectDetector};
use std::path::Path;

#[async_trait]
impl BaseAdapter for WebAdapter {}

impl CoreIdent for WebAdapter {
    fn core_id(&self) -> &'static str {
        "web"
    }

    fn name(&self) -> &str {
        "web"
    }

    fn ecosystem(&self) -> mgc_types::ecosystem::Ecosystem {
        mgc_types::ecosystem::Ecosystem::Web
    }
}

impl ProjectDetector for WebAdapter {
    fn can_handle(&self, project_root: &Path) -> bool {
        project_root.join("package.json").exists()
    }
}
