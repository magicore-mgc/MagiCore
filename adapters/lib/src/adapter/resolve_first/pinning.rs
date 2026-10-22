//! Map an unpinned request to the resolved version syntax of its lane.
//! Chuyển request chưa pin sang cú pháp version của lane đã resolve.

use crate::adapter::types::ResolveFirst;
use mgc_types::{MgResult, Version, VersionRange};

/// Preserve explicit ranges and pin unversioned requests to the selected version.
/// Giữ range tường minh và pin request chưa version vào bản đã chọn.
pub(super) fn pin_requested_range(
    lane: &ResolveFirst,
    requested: VersionRange,
    resolved: &Version,
) -> MgResult<VersionRange> {
    if !requested.is_star() {
        return Ok(requested);
    }

    match lane {
        ResolveFirst::Python => VersionRange::parse(&format!("=={resolved}")),
        ResolveFirst::Rust | ResolveFirst::Go | ResolveFirst::DotNet | ResolveFirst::JavaPom => {
            VersionRange::parse(&resolved.to_string())
        }
    }
}
