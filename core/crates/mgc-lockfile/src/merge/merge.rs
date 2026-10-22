// Three-way lockfile package merge implementation.
// Implementation merge package lockfile ba chiều.
use std::collections::BTreeSet;

use super::merge_conflict::MergeConflict;
use super::package_index::{PackageIdentity, PkgMap, index_pkgs};
use crate::Lockfile;

/// Merges `theirs` into `ours` relative to `base`.
/// Trộn `theirs` vào `ours` dựa trên `base`.
pub fn merge3(
    base: &Lockfile,
    ours: &Lockfile,
    theirs: &Lockfile,
) -> Result<Lockfile, MergeConflict> {
    let base_p: PkgMap = index_pkgs(&base.packages)?;
    let ours_p: PkgMap = index_pkgs(&ours.packages)?;
    let theirs_p: PkgMap = index_pkgs(&theirs.packages)?;

    let mut out = ours.clone();
    out.packages = vec![];

    let identities: BTreeSet<PackageIdentity> = base_p
        .keys()
        .cloned()
        .chain(ours_p.keys().cloned())
        .chain(theirs_p.keys().cloned())
        .collect();

    for identity in &identities {
        let b = base_p.get(identity);
        let o = ours_p.get(identity);
        let t = theirs_p.get(identity);
        match (b, o, t) {
            // A package added on one side is adopted from that side.
            // Package được thêm ở một nhánh thì nhận từ nhánh đó.
            (None, Some(o), None) => out.packages.push(o.clone()),
            (None, None, Some(t)) => out.packages.push(t.clone()),
            // If both branches retain the base version, keep ours' metadata.
            // Nếu hai nhánh giữ version gốc, giữ metadata từ ours.
            (Some(b), Some(o), Some(t)) if b.version == o.version && b.version == t.version => {
                out.packages.push(o.clone());
            }
            // Equal branch versions merge even when the base differs.
            // Hai nhánh cùng version thì merge dù version gốc khác.
            (Some(_), Some(o), Some(t)) if o.version == t.version => {
                out.packages.push(o.clone());
            }
            // Take the only changed version when the other side is unchanged.
            // Lấy version đã đổi nếu phía còn lại không thay đổi.
            (Some(b), Some(o), Some(t)) if o.version == b.version => {
                out.packages.push(t.clone());
            }
            (Some(b), Some(o), Some(t)) if t.version == b.version => {
                out.packages.push(o.clone());
            }
            // Divergent changes fail closed rather than choosing a branch.
            // Thay đổi phân kỳ phải lỗi, không tự chọn nhánh thắng.
            (Some(b), Some(o), Some(t)) => {
                return Err(MergeConflict {
                    name: identity.2.clone(),
                    base_version: Some(b.version.clone()),
                    ours_version: o.version.clone(),
                    theirs_version: t.version.clone(),
                });
            }
            // Removal wins when the other side kept the unchanged base.
            // Xóa thắng nếu phía kia giữ nguyên version gốc.
            (Some(b), None, Some(t)) if t.version == b.version => {}
            (Some(b), Some(o), None) if o.version == b.version => {}
            // A version bump wins over removal on the other side.
            // Version mới thắng thao tác xóa ở nhánh còn lại.
            (Some(_), None, Some(t)) => out.packages.push(t.clone()),
            (Some(_), Some(o), None) => out.packages.push(o.clone()),
            // Identical add/add entries merge; divergent entries conflict.
            // Add/add giống nhau thì merge; khác nhau thì báo conflict.
            (None, Some(o), Some(t)) if o.version == t.version => out.packages.push(o.clone()),
            (None, Some(o), Some(t)) => {
                return Err(MergeConflict {
                    name: identity.2.clone(),
                    base_version: None,
                    ours_version: o.version.clone(),
                    theirs_version: t.version.clone(),
                });
            }
            // An entry removed by both sides stays removed.
            // Entry bị xóa ở cả hai nhánh tiếp tục vắng mặt.
            _ => {}
        }
    }

    Ok(out)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
