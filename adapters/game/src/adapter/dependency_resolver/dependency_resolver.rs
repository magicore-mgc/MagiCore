use super::super::GameAdapter;
use async_trait::async_trait;
use mgc_types::adapter::{AddOptions, UpdatedPackage};
use mgc_types::capabilities::DependencyResolver;
use mgc_types::{MgResult, PackageId, PackageName, VersionRange};
use std::path::Path;

#[async_trait]
impl DependencyResolver for GameAdapter {
    async fn add(
        &self,
        _project_root: &Path,
        _name: &PackageName,
        _range: Option<&VersionRange>,
        _opts: AddOptions,
    ) -> MgResult<PackageId> {
        Err(mgc_types::capabilities::unsupported_capability(
            "game",
            "add",
            "the GameAdapter has no MagiCore-owned resolver/writer for this ecosystem; dependency addition is unsupported",
        ))
    }

    async fn remove(&self, _project_root: &Path, _name: &PackageName) -> MgResult<()> {
        Err(mgc_types::capabilities::unsupported_capability(
            "game",
            "remove",
            "the GameAdapter has no MagiCore-owned dependency remover for this ecosystem; dependency removal is unsupported",
        ))
    }

    async fn update(
        &self,
        _project_root: &Path,
        _name: Option<&PackageName>,
    ) -> MgResult<Vec<UpdatedPackage>> {
        Err(mgc_types::capabilities::unsupported_capability(
            "game",
            "update",
            "the GameAdapter has no MagiCore-owned dependency updater for this ecosystem; dependency updates are unsupported",
        ))
    }
}
