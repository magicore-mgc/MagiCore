//! React Native package.json manifest parsing through the shared JS service.
//! Parse manifest package.json của React Native qua service JS dùng chung.
//!
//! Owner (S08/D014): `mgc-js-service::manifest` — the same native parser
//! the web core uses, addressed directly (no adapter-to-adapter edge, R07).
//! Chủ sở hữu: `mgc-js-service::manifest` — cùng parser native core web
//! dùng, gọi trực tiếp (không cạnh adapter-to-adapter, R07).

use mgc_types::{Manifest, MgResult};
use std::path::Path;

/// Parse package.json using the shared native JS manifest engine.
/// Parse package.json bằng engine manifest JS native dùng chung.
pub fn parse_package_json(project_root: &Path) -> MgResult<Manifest> {
    mgc_js_service::manifest::parse_manifest(project_root)
}

/// Write package.json using the shared native JS manifest writer.
/// Ghi package.json bằng bộ viết manifest JS native dùng chung.
pub fn write_package_json(project_root: &Path, manifest: &Manifest) -> MgResult<()> {
    mgc_js_service::manifest::write_manifest(project_root, manifest)
}
