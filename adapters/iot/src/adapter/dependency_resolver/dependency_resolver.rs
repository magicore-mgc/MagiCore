//! Preserves explicit unsupported add/remove/update results. Dependency operations are never delegated to pio or west. — Giữ kết quả unsupported rõ ràng cho add/remove/update. Không ủy quyền dependency operation cho pio hoặc west.

use std::path::Path;

use super::super::IotAdapter;
use async_trait::async_trait;
use mgc_types::adapter::{AddOptions, UpdatedPackage};
use mgc_types::capabilities::DependencyResolver;
use mgc_types::{MgResult, PackageId, PackageName, VersionRange};
#[async_trait]
impl DependencyResolver for IotAdapter {
    async fn add(
        &self,
        project_root: &Path,
        name: &PackageName,
        range: Option<&VersionRange>,
        opts: AddOptions,
    ) -> MgResult<PackageId> {
        // No dependency operation is delegated from this adapter.
        // Adapter này không ủy quyền thao tác dependency ra công cụ ngoài.
        let _ = (project_root, name, range, opts);
        Err(mgc_types::capabilities::unsupported_capability(
            "iot",
            "add",
            "the IoT adapter has no MagiCore-owned resolver/writer for this ecosystem; dependency addition is unsupported",
        ))
    }

    async fn remove(&self, _project_root: &Path, _name: &PackageName) -> MgResult<()> {
        Err(mgc_types::capabilities::unsupported_capability(
            "iot",
            "remove",
            "the IoT adapter has no MagiCore-owned dependency remover for this ecosystem; dependency removal is unsupported",
        ))
    }

    async fn update(
        &self,
        _project_root: &Path,
        _name: Option<&PackageName>,
    ) -> MgResult<Vec<UpdatedPackage>> {
        Err(mgc_types::capabilities::unsupported_capability(
            "iot",
            "update",
            "the IoT adapter has no MagiCore-owned dependency updater for this ecosystem; dependency updates are unsupported",
        ))
    }
}
