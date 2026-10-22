//! Cargo manifest bridge.
//! Cầu nối manifest Cargo.

use mgc_types::{Ecosystem, Manifest, MgResult};
use std::path::Path;

/// MGC models PEP 735's conventional `dev` group only. Other groups, group
/// includes, and non-simple PEP 508 requirements remain outside the native
/// mutation lane until their selection semantics can be preserved.
/// (MGC chỉ mô hình nhóm `dev`; nhóm khác/include/cú pháp chưa hỗ trợ phải
/// fail-closed để không làm sai ý nghĩa khai báo.)
pub(crate) fn parse_cargo_manifest(root: &Path) -> MgResult<Manifest> {
    mgc_adapter_base::cargo_manifest::parse_manifest(root, Ecosystem::Lib)
}

pub(crate) fn write_cargo_manifest(root: &Path, manifest: &Manifest) -> MgResult<()> {
    mgc_adapter_base::cargo_manifest::write_manifest(root, manifest)
}
