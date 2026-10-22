//! Acquire the project mutation gateway before a standalone install.
//! Lấy mutation gateway của project trước khi install độc lập.

use super::*;

pub async fn install_with_adapter(
    adapter: &dyn PackageAdapter,
    root: &Path,
    add_cmd: &str,
    frozen: bool,
    opts: mgc_types::adapter::InstallOptions,
) -> Result<()> {
    // Every standalone install runs under the writer lock (P0): the
    // crash-journal recovery below must never race a concurrent remove.
    // Callers already holding the guard (mutation tails) use the _locked
    // variant — the OS lock is not same-process re-entrant. Recovery runs
    // inside, so installs never build on unrestored state.
    // (Install độc lập qua gateway — lock + recovery.)
    let write_lock = begin_dependency_mutation(adapter, root, MutationOperation::Install).await?;
    install_with_adapter_locked(adapter, root, add_cmd, frozen, opts, &write_lock).await
}
