// Filesystem tree metrics
// Đo số file và dung lượng của cache tree.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn inspect_tree(root: &Path) -> anyhow::Result<TreeStats> {
    if !root.exists() {
        return Ok(TreeStats::default());
    }

    let mut files = 0usize;
    let mut bytes = 0u64;
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        files += 1;
        bytes += entry.metadata()?.len();
    }

    Ok(TreeStats { files, bytes })
}
