//! go.sum skeleton writer.
//! Writer skeleton go.sum.

use mgc_types::MgResult;

use crate::ecosystem_tag::EcosystemTag;
use crate::schema::Lockfile;

use super::super::shared::require_version;

/// go.sum skeleton: comment-only placeholder — go regenerates the real
/// hashes from go.mod + module cache, so nothing non-comment is emitted.
/// go.sum skeleton: chỉ toàn comment — go sinh lại hash thật từ go.mod +
/// module cache, nên không xuất dòng nào ngoài comment.
pub fn export_go_sum(lock: &Lockfile) -> MgResult<Vec<u8>> {
    let mut out = String::from("# skeleton — go toolchain regenerates\n");
    for pkg in lock
        .packages
        .iter()
        .filter(|p| p.ecosystem == EcosystemTag::Go)
    {
        let version = require_version(pkg)?;
        // Still comment-only: module pins are informational, never parsed.
        // Vẫn chỉ là comment: pin module chỉ để tham chiếu, không được parse.
        out.push_str(&format!("# module {}@{version}\n", pkg.name));
    }
    Ok(out.into_bytes())
}
