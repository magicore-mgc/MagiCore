//! Owns IoT adapter state, its public framework/board/target methods and the exact capability manifest. Sibling trait owners receive only subtree-private access to the selected framework field. — Sở hữu state IoT adapter, method public framework/board/target và capability manifest chính xác. Owner trait sibling chỉ được truy cập field framework trong subtree này.

use std::path::Path;

use crate::framework::{IotFramework, detect_framework, target_from_manifest};
use mgc_types::capabilities::Capability;
pub struct IotAdapter {
    pub(super) framework: IotFramework,
}
impl IotAdapter {
    /// Capability manifest (Global Gate 1) — code-reality notes:
    /// - ProjectDetector: `detect_framework`/`manifest_is_iot` — real.
    /// - ScaffoldProvider: src/scaffold (esp32-rust/platformio/zephyr
    ///   generators) + the `mgc create-iot` CLI lane — real.
    /// - ContentStoreProvider is not claimed: esp32-rust uses the shared
    ///   native Lib/Rust engine in the CLI; PlatformIO/Zephyr are delegated.
    /// - AuditProvider: shared polyglot dispatch — real.
    ///
    /// DependencyResolver/LockfileProvider NOT claimed: resolve is
    /// fail-closed and write_manifest is real only for esp32-rust.
    /// Bảng capability (Global Gate 1) — ghi chú theo code thật.
    pub const CAPABILITIES: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::AuditProvider,
    ];
}
impl IotAdapter {
    pub fn detect(root: &Path) -> Option<Self> {
        let framework = detect_framework(root)?;
        Some(Self { framework })
    }

    pub fn framework(&self) -> &'static str {
        self.framework.as_str()
    }

    pub fn board(&self, root: &Path) -> Option<String> {
        // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
        if let Ok(Some(content)) =
            mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
            && let Ok(v) = toml::from_str::<toml::Value>(&content)
        {
            return v
                .get("iot")
                .and_then(|i| i.get("board"))
                .and_then(|b| b.as_str())
                .map(str::to_string);
        }
        None
    }

    pub fn target(&self, root: &Path) -> Option<String> {
        target_from_manifest(root)
    }
}
