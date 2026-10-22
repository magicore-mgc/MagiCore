//! Preserves native manifest writing only for ESP32-Rust; PlatformIO and Zephyr mutations stay unsupported. — Giữ ghi manifest native chỉ cho ESP32-Rust; mutation PlatformIO và Zephyr vẫn unsupported.

use std::path::Path;

use super::super::IotAdapter;
use crate::framework::IotFramework;
use async_trait::async_trait;
use mgc_types::capabilities::LockfileProvider;
use mgc_types::{Manifest, MgResult};
#[async_trait]
impl LockfileProvider for IotAdapter {
    async fn write_manifest(&self, project_root: &Path, manifest: &Manifest) -> MgResult<()> {
        match self.framework {
            IotFramework::Esp32Rust => {
                mgc_adapter_base::cargo_manifest::write_manifest(project_root, manifest)
            }
            IotFramework::Platformio | IotFramework::Zephyr => {
                // Fail-closed (hardware precedent): the old silent Ok no-op
                // faked a manifest write that never happened.
                // Fail-closed (tiền lệ hardware): Ok no-op cũ giả một lần
                // ghi manifest không bao giờ xảy ra.
                Err(mgc_types::MgError::Unsupported {
                    core: "iot",
                    capability: "write_manifest",
                    guidance: format!(
                        "{} projects do not have a MagiCore-owned dependency manifest; dependency mutation is unsupported",
                        self.framework.as_str()
                    ),
                })
            }
        }
    }
}
