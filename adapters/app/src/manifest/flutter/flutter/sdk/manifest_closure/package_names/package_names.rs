//! Project reachable Flutter SDK package names from the validated closure.
//! Lấy tên package Flutter SDK từ closure đã kiểm tra.

use super::super::declared_dependencies::declared_flutter_sdk_dependencies;
use super::super::dependency_closure::read_flutter_sdk_dependency_closure;
use crate::manifest::flutter::FlutterResult;
use mgc_types::{Ecosystem, Manifest};
use std::path::Path;

/// Return sorted package names reachable from project SDK declarations.
/// Trả danh sách tên đã sắp xếp, đạt tới từ khai báo SDK của dự án.
pub(crate) fn flutter_sdk_package_names(
    project_root: &Path,
    sdk_root: &Path,
) -> FlutterResult<Vec<String>> {
    let declared = declared_flutter_sdk_dependencies(project_root)?;
    if declared.is_empty() {
        return Ok(Vec::new());
    }
    let (names, _) = read_flutter_sdk_dependency_closure(
        project_root,
        sdk_root,
        &Manifest::new("sdk", Ecosystem::App),
        declared,
    )?;
    Ok(names)
}
