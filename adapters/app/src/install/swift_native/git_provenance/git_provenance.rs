//! Read the first locked Git commit marker for a package identity.
//! Đọc marker commit Git đầu tiên trong lock theo identity package.

/// Preserve exact package-name matching and first `git-commit:` marker behavior.
/// Giữ so khớp tên package chính xác và lấy marker `git-commit:` đầu tiên.
pub(in crate::install) fn git_commit_for_package(
    lock_packages: &[mgc_lockfile::Package],
    package_name: &str,
) -> Option<String> {
    lock_packages
        .iter()
        .find(|package| package.name == package_name)
        .and_then(|package| package.markers.as_ref())
        .and_then(|markers| {
            markers
                .iter()
                .find_map(|marker| marker.strip_prefix("git-commit:").map(str::to_string))
        })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
