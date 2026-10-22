//! Preserves Web delegation and the typed fail-closed refusal for Terraform manifest writes. — Giữ ủy quyền Web và lỗi từ chối fail-closed có kiểu khi ghi manifest Terraform.

use async_trait::async_trait;
use std::path::Path;

use mgc_types::capabilities::LockfileProvider;
use mgc_types::{Manifest, MgResult};

use super::super::CloudAdapter;

#[async_trait]
impl LockfileProvider for CloudAdapter {
    async fn write_manifest(&self, project_root: &Path, manifest: &Manifest) -> MgResult<()> {
        if let Some(web) = &self.web {
            return web.write_manifest(project_root, manifest).await;
        }
        // Fail-closed (hardware precedent): the old silent Ok no-op faked
        // a manifest write that never happened for terraform projects.
        // Fail-closed (tiền lệ hardware): Ok no-op cũ giả một lần ghi
        // manifest không bao giờ xảy ra cho project terraform.
        Err(mgc_types::MgError::Unsupported {
            core: "cloud",
            capability: "write_manifest",
            guidance: "terraform projects do not use mgc-written manifests; \
                       write resources in HCL directly or scaffold via `mgc create-clo`"
                .to_string(),
        })
    }
}
