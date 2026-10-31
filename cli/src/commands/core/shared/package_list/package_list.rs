use super::*;

/// Print the installed inventory, one line per package.
///
/// Each line carries the package id, the manifest section it was declared
/// in (when the adapter knows it) and a `dev` marker kept for scripts that
/// already parse the old suffix. Listing must never hide a declared
/// dependency because of the section it lives in.
/// (In inventory đã cài, mỗi package một dòng. Mỗi dòng có id, section
/// manifest đã khai (khi adapter biết) và marker `dev` giữ cho script đang
/// parse hậu tố cũ. `list` không bao giờ ẩn dependency theo section.)
#[allow(dead_code)]
pub async fn list(adapter: &dyn PackageAdapter, root: &Path) -> Result<()> {
    let mut packages = adapter.list(root).await?;
    if packages.is_empty() {
        info("No packages installed");
        return Ok(());
    }
    // Stable, human-scannable order independent of adapter enumeration.
    // (Thứ tự ổn định, dễ quét, không phụ thuộc adapter.)
    packages.sort_by(|left, right| {
        left.id
            .name_str()
            .cmp(right.id.name_str())
            .then_with(|| left.id.version().cmp(right.id.version()))
    });
    for pkg in &packages {
        let dev = if pkg.is_dev { " (dev)" } else { "" };
        let section = pkg
            .section
            .as_deref()
            .map(|section| format!(" [{section}]"))
            .unwrap_or_default();
        info(&format!(
            "  {}@{}{}{}",
            pkg.id.name_str(),
            pkg.id.version(),
            section,
            dev
        ));
    }
    Ok(())
}
