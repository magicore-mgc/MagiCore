//! Native resolve-first dependency-add preparation for LibAdapter.
//! Chuẩn bị thêm dependency qua resolve-first native cho LibAdapter.

use crate::adapter::types::LibAdapter;
use mgc_types::adapter::{AddOptions, PreparedAdd};
use mgc_types::{MgResult, PackageName, VersionRange};
use std::path::Path;

pub(in crate::adapter::package_adapter) async fn prepare_add(
    adapter: &LibAdapter,
    project_root: &Path,
    name: &PackageName,
    range: Option<&VersionRange>,
    opts: AddOptions,
) -> MgResult<PreparedAdd> {
    adapter.require_python_native_at(project_root, "add")?;
    // Resolve-first (C0 FIX2 + native-add): the pyproject/Cargo writers
    // cannot persist star ranges (every saved dep needs a bound), so
    // booking an unpinned dep in memory faked a mutation the disk never
    // saw. Resolve the real version natively FIRST for EVERY range —
    // star or explicit — through the shared native lane below (no
    // toolchain is spawned on any path); a resolve failure errors
    // honestly instead of fake-adding.
    // (Resolve-trước mọi range qua lane native dùng chung — không
    // spawn toolchain; fail trung thực thay vì add giả.)
    if let Some(prepared) = adapter
        .resolve_first_add(project_root, name, range, &opts)
        .await?
    {
        return Ok(prepared);
    }
    Err(mgc_types::capabilities::unsupported_capability(
        "lib",
        "prepare add",
        "this language has no native resolve-first add lane; no placeholder package version is returned",
    ))
}
