//! Fail-closed CocoaPods Podfile handling.
//! Xử lý Podfile CocoaPods theo fail-closed.

use crate::manifest::flutter::{FlutterError, FlutterResult};
use mgc_types::Manifest;
use std::path::Path;

/// Parse CocoaPods Podfile (for ObjC).
pub(in crate::manifest::flutter::flutter) fn parse_podfile(
    project_root: &Path,
) -> FlutterResult<Manifest> {
    let podfile_path = project_root.join("Podfile");
    if !podfile_path.exists() {
        return Err(FlutterError::Other("Podfile not found".to_string()));
    }

    Err(FlutterError::Unsupported {
        capability: "parse-podfile",
        guidance: format!(
            "{} is executable Ruby DSL; MagiCore has no safe CocoaPods manifest parser",
            podfile_path.display()
        ),
    })
}

/// Write Manifest back to Podfile.
pub(in crate::manifest::flutter::flutter) fn write_podfile(
    _project_root: &Path,
    _manifest: &Manifest,
) -> FlutterResult<()> {
    Err(FlutterError::Unsupported {
        capability: "write-podfile",
        guidance: "MagiCore does not rewrite executable Ruby Podfile content".to_string(),
    })
}
