// Shared cache isolation
// Cô lập shared cache root theo từng scenario mà không sửa process environment.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn with_isolated_shared_cache<T, F>(
    shared_root: Option<&Path>,
    f: F,
) -> anyhow::Result<T>
where
    F: FnOnce(&Path) -> anyhow::Result<T>,
{
    if let Some(shared_root) = shared_root {
        return f(shared_root);
    }
    let isolated = tempfile::tempdir()?;
    f(isolated.path())
}
