//! Coordinates Dart metadata generation for native Flutter install.
//! Điều phối tạo metadata Dart cho install Flutter native.

use mgc_types::{MgResult, ResolvedGraph};
use std::path::Path;

use super::package_config::write_flutter_package_config_with_sdk_root;

pub(in crate::install) fn write_flutter_package_config(
    graph: &ResolvedGraph,
    project_root: &Path,
    pub_cache: &Path,
) -> MgResult<()> {
    let (sdk_root, _) = crate::manifest::flutter::flutter_sdk_packages(project_root)?;
    write_flutter_package_config_with_sdk_root(graph, project_root, pub_cache, sdk_root.as_deref())
}
