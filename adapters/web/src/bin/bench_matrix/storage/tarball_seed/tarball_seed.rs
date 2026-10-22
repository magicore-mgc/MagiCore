use crate::bench_matrix::storage::build_tarball_bytes;
// Benchmark cache tarball seeding
// Ghi tarball fixture vào cache local hoặc shared theo layout.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn seed_local_tarballs(
    project_root: &Path,
    packages: &[ResolvedPackage],
    files_per_package: usize,
) -> anyhow::Result<()> {
    let layout = Layout::new(project_root.join(".magicore").join("cache").join("web"));
    seed_tarballs_into_layout(&layout, packages, files_per_package)
}

pub(in crate::bench_matrix) fn seed_shared_tarballs(
    shared_root: &Path,
    packages: &[ResolvedPackage],
    files_per_package: usize,
) -> anyhow::Result<()> {
    let layout = Layout::new(shared_root.to_path_buf());
    seed_tarballs_into_layout(&layout, packages, files_per_package)
}

pub(in crate::bench_matrix) fn seed_tarballs_into_layout(
    layout: &Layout,
    packages: &[ResolvedPackage],
    files_per_package: usize,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(layout.root())?;
    let cache = PackageCache::new(layout.cache_dir())?;
    for pkg in packages {
        let tarball_path = cache.tarball_path(&pkg.id);
        if let Some(parent) = tarball_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(
            tarball_path,
            build_tarball_bytes(&pkg.id, files_per_package)?,
        )?;
    }
    Ok(())
}
