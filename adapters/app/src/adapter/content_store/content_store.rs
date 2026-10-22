//! Native install lane and content-store capability.
//! Lane install native và capability content-store.

use crate::adapter::AppAdapter;
use crate::language::AppLanguage;
use async_trait::async_trait;
use mgc_types::MgResult;
use mgc_types::adapter::{InstallOptions, InstallSummary};
use mgc_types::capabilities::ContentStoreProvider;
use std::path::Path;

#[async_trait]
impl ContentStoreProvider for AppAdapter {
    fn probe_content_store(&self) -> MgResult<()> {
        match self.language {
            AppLanguage::Flutter | AppLanguage::Swift => Ok(()),
            AppLanguage::ReactNative => Err(mgc_types::capabilities::unsupported_capability(
                "app",
                "content_store",
                "React Native install is not available until JS, Android and iOS artifacts can be committed atomically",
            )),
            AppLanguage::Kotlin
            | AppLanguage::ObjC
            | AppLanguage::Multi
            | AppLanguage::Maui
            | AppLanguage::Tauri => Err(mgc_types::capabilities::unsupported_capability(
                "app",
                "content_store",
                "this ecosystem has no MagiCore-native install path",
            )),
        }
    }

    async fn install(
        &self,
        graph: &mgc_types::ResolvedGraph,
        project_root: &Path,
        opts: InstallOptions,
    ) -> MgResult<InstallSummary> {
        if !matches!(self.language, AppLanguage::Flutter | AppLanguage::Swift) {
            return Err(mgc_types::capabilities::unsupported_capability(
                "app",
                "install",
                "this app ecosystem has no complete MagiCore-owned native dependency lifecycle",
            ));
        }
        // Use new install pipeline (pending native-resolve lock entries are
        // consumed here — same-instance flow).
        // (Dùng install pipeline mới (entry lock từ resolve native được
        // tiêu thụ ở đây — flow cùng instance).)
        let mut pending = self.pending_install.take()?;
        if opts.replay_from_lock {
            pending.lock_packages.clear();
            pending.swift_protocol = None;
        }
        crate::install::run_install_with_swift_protocol(
            self.language,
            graph,
            project_root,
            opts,
            None,
            pending.lock_packages,
            pending.swift_protocol,
        )
        .await
    }
}
