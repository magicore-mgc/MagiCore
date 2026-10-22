use super::MaterializeResult;
use crate::protocols::ResolvedEntry;
use std::path::PathBuf;

pub(in crate::protocols::crates) fn materialize(
    entry: &ResolvedEntry,
    bytes: &[u8],
    cargo_home: &std::path::Path,
) -> MaterializeResult<PathBuf> {
    let cache_dir = cargo_home.join("registry").join("cache");
    std::fs::create_dir_all(&cache_dir)?;
    let crate_path = cache_dir.join(format!("{}-{}.crate", entry.name, entry.version));
    std::fs::write(&crate_path, bytes)?;

    let src_root = cargo_home.join("registry").join("src");
    crate::protocols::archive::extract_crate_tar_gz(
        bytes,
        &src_root,
        &format!("{}-{}", entry.name, entry.version),
    )?;
    Ok(crate_path)
}
